import { useSliceStore } from '../../store/sliceStore';
import styles from './HomeButtons.module.css';

export function HomeButtons() {
  const activeSlice = useSliceStore((state) => state.activeSlice);

  if (!activeSlice) return null;

  return (
    <div className={styles.buttons_container}>
      <button
        className={`${styles.btn} ${styles.btn1} ${activeSlice === 'cortes' ? styles.active : ''}`}
      >
        CORTES
      </button>
      <button
        className={`${styles.btn} ${styles.btn2} ${activeSlice === 'produtos' ? styles.active : ''}`}
      >
        PRODUTOS
      </button>
      <button
        className={`${styles.btn} ${styles.btn3} ${activeSlice === 'contato' ? styles.active : ''}`}
      >
        CONTATO
      </button>
    </div>
  );
}
