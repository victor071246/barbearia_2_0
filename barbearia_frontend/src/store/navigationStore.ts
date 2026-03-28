import { create } from 'zustand';

type Page = 'home' | 'items' | 'contact';

interface NavigationState {
  currentPage: Page;
  setPage: (page: Page) => void;
  categoria: string;
  setCategoria: (categoria: string) => void;
}

export const useNavigationStore = create<NavigationState>((set) => ({
  currentPage: 'home',
  setPage: (page) => set({ currentPage: page }),
  categoria: 'todos',
  setCategoria: (categoria) => set({ categoria }),
}));
