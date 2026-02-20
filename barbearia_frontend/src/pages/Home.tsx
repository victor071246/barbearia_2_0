import { useState } from 'react';
import styles from './Home.module.css';

export function Home() {
  const [active, setActive] = useState<string | null>(null);

  return (
    <div className={styles.container}>
      <div className={styles.second_box}>
        <div className={styles.box}>
          <div className={styles.grid}>
            <div
              className={`${styles.slice} ${styles.sliceCortes} ${active && active !== 'cortes' ? '' : 'styles.dimmed'}`}
              onClick={() => setActive('cortes')}
            >
              <div className={styles.slice_content}>
                <img
                  src="https://images.unsplash.com/photo-1503951914875-452162b0f3f1?w=600"
                  alt="Cortes"
                />
              </div>
            </div>
            <div
              className={`${styles.slice} ${styles.sliceProdutos} ${active && active !== 'produtos' ? styles.dimmed : ''}`}
              onClick={() => setActive('produtos')}
            >
              <div className={styles.slice_content}>
                <img
                  src="https://images.unsplash.com/photo-1599351431202-1e0f0137899a?w=600"
                  alt="Produtos"
                />
              </div>
            </div>
            <div
              className={`${styles.slice} ${styles.sliceContato} ${active && active !== 'contato' ? styles.dimmed : ''}`}
              onClick={() => setActive('contato')}
            >
              <div className={styles.slice_content}>
                <img
                  src="https://images.unsplash.com/photo-1521490683712-35a1cb235d1c?w=600"
                  alt="Contato"
                />
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}
