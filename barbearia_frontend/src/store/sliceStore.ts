import { create } from 'zustand';

type Slice = 'cortes' | 'produtos' | 'contato' | null;

interface SliceState {
  activeSlice: Slice;
  setActiveSlice: (slice: Slice) => void;
}

export const useSliceStore = create<SliceState>((set) => ({
  activeSlice: null,
  setActiveSlice: (slice) => set({ activeSlice: slice }),
}));
