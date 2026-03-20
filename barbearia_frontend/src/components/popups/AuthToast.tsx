import { useEffect } from 'react';
import styles from './AuthToast.module.css';

interface AuthToastProps {
  show: boolean;
  onDone: () => void;
}

export function AuthToast({ show, onDone }: AuthToastProps) {
  useEffect(() => {
    if (show) {
      const timer = setTimeout(onDone, 2000);
      return () => clearTimeout(timer);
    }
  }, [show, onDone]);

  if (!show) return null;

  return (
    <div className={styles.overlay}>
      <div className={styles.popup}>
        <div className={styles.corner}>
          <span className={styles.text}>Logado com sucesso</span>
        </div>
      </div>
    </div>
  );
}
