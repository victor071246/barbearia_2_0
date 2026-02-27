import { useSliceStore } from '../../store/sliceStore';
import styles from './HomeButtons.module.css';

export function HomeButtons() {
  const activeSlice = useSliceStore((state) => state.activeSlice);

  if (!activeSlice) return null;

  return (
    <div className={styles.buttons_container}>
      <button
        className={`${styles.btn} ${activeSlice === 'cortes' ? styles.active : ''}`}
      >
        CORTES
      </button>
      <button
        className={`${styles.btn} ${activeSlice === 'produtos' ? styles.active : ''}`}
      >
        PRODUTOS
      </button>
      <button
        className={`${styles.btn} ${activeSlice === 'contato' ? styles.active : ''}`}
      >
        CONTATO
      </button>
      m
    </div>
  );
}
