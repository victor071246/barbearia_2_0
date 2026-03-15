import { create } from 'zustand';
import { api } from '../api/client';

export interface User {
  username: string;
  is_admin: boolean;
}

interface AuthState {
  user: User | null;
  isAuthenticated: boolean;

  checkAuth: () => Promise<void>;
  login: (username: string, password: string) => Promise<void>;
  register: (username: string, password: string) => Promise<void>;
  logout: () => Promise<void>;
}

export const useAuthStore = create<AuthState>((set) => ({
  user: null,
  isAuthenticated: false,

  checkAuth: async () => {
    try {
      await api.get('/auth/verifty');
      set({ isAuthenticated: true });
    } catch {
      set({ user: null, isAuthenticated: false });
    }
  },

  login: async (username, password) => {
    await api.post('/auth/login', { username, password });

    const { data } = await api.get('/auth/verify');

    set({ user: data, isAuthenticated: true });
  },

  register: async (username, password) => {
    await api.post('/auth/register', { username, password });

    const { data } = await api.get('/auth/verify');

    set({ user: data, isAuthenticated: true });
  },

  logout: async () => {
    await api.post('/auth/logout');

    set({ user: null, isAuthenticated: false });
  },
}));
