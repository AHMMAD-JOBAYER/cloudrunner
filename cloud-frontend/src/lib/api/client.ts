import { browser } from '$app/environment';
import { goto } from '$app/navigation';

const API_URL = import.meta.env.VITE_API_URL || 'http://localhost:8080';

export interface ApiResponse<T> {
  success: boolean;
  message: string;
  data?: T;
}

export interface Teacher {
  id: string;
  email: string;
  full_name: string;
  department: string | null;
}

export interface LoginResponse {
  token: string;
  teacher: Teacher;
}

export interface NixOsConfig {
  id: string;
  filename: string;
  content: string;
  file_size: number;
  created_at: string;
  updated_at: string;
}

class ApiClient {
  private getToken(): string | null {
    if (!browser) return null;
    return localStorage.getItem('token');
  }

  private async request<T>(
    endpoint: string,
    options: RequestInit = {}
  ): Promise<T> {
    const token = this.getToken();
    const headers: HeadersInit = {
      'Content-Type': 'application/json',
      ...options.headers,
    };

    if (token) {
      headers['Authorization'] = `Bearer ${token}`;
    }

    const response = await fetch(`${API_URL}${endpoint}`, {
      ...options,
      headers,
    });

    if (response.status === 401) {
      // Unauthorized - clear token and redirect to login
      if (browser) {
        localStorage.removeItem('token');
        localStorage.removeItem('teacher');
        goto('/login');
      }
      throw new Error('Unauthorized');
    }

    if (!response.ok) {
      const error = await response.json().catch(() => ({ message: 'An error occurred' }));
      throw new Error(error.message || `HTTP error! status: ${response.status}`);
    }

    return response.json();
  }

  // Auth endpoints
  async register(data: {
    email: string;
    password: string;
    full_name: string;
    department?: string;
  }): Promise<ApiResponse<Teacher>> {
    return this.request('/api/auth/register', {
      method: 'POST',
      body: JSON.stringify(data),
    });
  }

  async login(email: string, password: string): Promise<ApiResponse<LoginResponse>> {
    const response = await this.request<ApiResponse<LoginResponse>>('/api/auth/login', {
      method: 'POST',
      body: JSON.stringify({ email, password }),
    });

    // Store token and teacher data
    if (browser && response.data) {
      localStorage.setItem('token', response.data.token);
      localStorage.setItem('teacher', JSON.stringify(response.data.teacher));
    }

    return response;
  }

  async logout(): Promise<void> {
    try {
      await this.request('/api/auth/logout', { method: 'POST' });
    } finally {
      if (browser) {
        localStorage.removeItem('token');
        localStorage.removeItem('teacher');
      }
    }
  }

  async getCurrentTeacher(): Promise<ApiResponse<Teacher>> {
    return this.request('/api/auth/me');
  }

  async requestPasswordReset(email: string): Promise<ApiResponse<void>> {
    return this.request('/api/auth/reset-password', {
      method: 'POST',
      body: JSON.stringify({ email }),
    });
  }

  async confirmPasswordReset(token: string, new_password: string): Promise<ApiResponse<void>> {
    return this.request('/api/auth/reset-password/confirm', {
      method: 'POST',
      body: JSON.stringify({ token, new_password }),
    });
  }

  // Config endpoints
  async uploadConfig(filename: string, content: string): Promise<ApiResponse<NixOsConfig>> {
    return this.request('/api/configs', {
      method: 'POST',
      body: JSON.stringify({ filename, content }),
    });
  }

  async listConfigs(): Promise<ApiResponse<NixOsConfig[]>> {
    return this.request('/api/configs');
  }

  async getConfig(id: string): Promise<ApiResponse<NixOsConfig>> {
    return this.request(`/api/configs/${id}`);
  }

  async updateConfig(id: string, content: string): Promise<ApiResponse<NixOsConfig>> {
    return this.request(`/api/configs/${id}`, {
      method: 'PUT',
      body: JSON.stringify({ content }),
    });
  }

  async deleteConfig(id: string): Promise<ApiResponse<void>> {
    return this.request(`/api/configs/${id}`, {
      method: 'DELETE',
    });
  }
}

export const api = new ApiClient();
