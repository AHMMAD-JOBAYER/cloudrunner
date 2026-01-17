import { writable } from 'svelte/store';
import { browser } from '$app/environment';
import type { Teacher } from '$lib/api/client';

interface AuthState {
  isAuthenticated: boolean;
  teacher: Teacher | null;
  token: string | null;
}

function createAuthStore() {
  const initialState: AuthState = {
    isAuthenticated: false,
    teacher: null,
    token: null,
  };

  // Load from localStorage if in browser
  if (browser) {
    const token = localStorage.getItem('token');
    const teacherData = localStorage.getItem('teacher');

    if (token && teacherData) {
      try {
        initialState.token = token;
        initialState.teacher = JSON.parse(teacherData);
        initialState.isAuthenticated = true;
      } catch (e) {
        // Invalid data, clear it
        localStorage.removeItem('token');
        localStorage.removeItem('teacher');
      }
    }
  }

  const { subscribe, set, update } = writable<AuthState>(initialState);

  return {
    subscribe,
    login: (token: string, teacher: Teacher) => {
      if (browser) {
        localStorage.setItem('token', token);
        localStorage.setItem('teacher', JSON.stringify(teacher));
      }
      set({ isAuthenticated: true, teacher, token });
    },
    logout: () => {
      if (browser) {
        localStorage.removeItem('token');
        localStorage.removeItem('teacher');
      }
      set({ isAuthenticated: false, teacher: null, token: null });
    },
    updateTeacher: (teacher: Teacher) => {
      if (browser) {
        localStorage.setItem('teacher', JSON.stringify(teacher));
      }
      update(state => ({ ...state, teacher }));
    },
  };
}

export const authStore = createAuthStore();
