import { Link } from 'react-router-dom';
import { useNavigationStore } from '../../store/navigationStore';
import styles from './NavBar.module.css';

export function NavBar() {
  const { currentPage, setPage } = useNavigationStore();

  return (
    <nav className={styles.navbar}>
      <Link
        to="/"
        className={`${currentPage === 'home' ? styles.active : ''} ${styles.navbar_select_box}`}
      >
        Home
      </Link>
      <Link
        to="/servicos"
        className={`${currentPage === 'items' ? styles.active : ''} ${styles.navbar_select_box}`}
        onClick={() => {
          setPage('items');
        }}
      ></Link>
      <Link
        to="/contato"
        className={`${currentPage === 'contact' ? styles.active : ''} ${styles.navbar_select_box}`}
        onClick={() => setPage('contact')}
      ></Link>
    </nav>
  );
}
