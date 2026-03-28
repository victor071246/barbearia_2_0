import { useSliceStore } from '../../store/sliceStore';
import { useNavigate } from 'react-router-dom';
import { useNavigationStore } from '../../store/navigationStore';
import styles from './HomeButtons.module.css';

export function HomeButtons() {
  const activeSlice = useSliceStore((state) => state.activeSlice);
  const setCategoria = useNavigationStore((state) => state.setCategoria);
  const setPage = useNavigationStore((state) => state.setPage);
  const navigate = useNavigate();

  if (!activeSlice) return null;

  return (
    <div className={styles.buttons_container}>
      <button
        className={`${styles.btn} ${styles.btn1} ${activeSlice === 'cortes' ? styles.active : ''}`}
        onClick={() => {
          navigate('/servicos');
          setCategoria('servico');
          setPage('items');
        }}
      >
        CORTES
      </button>
      <button
        className={`${styles.btn} ${styles.btn2} ${activeSlice === 'produtos' ? styles.active : ''}`}
        onClick={() => {
          navigate('/servicos');
          setCategoria('produto');
          setPage('items');
        }}
      >
        PRODUTOS
      </button>
      <button
        className={`${styles.btn} ${styles.btn3} ${activeSlice === 'contato' ? styles.active : ''}`}
        onClick={() => {
          navigate('/contato');
          setPage('contact');
        }}
      >
        CONTATO
      </button>
    </div>
  );
}
