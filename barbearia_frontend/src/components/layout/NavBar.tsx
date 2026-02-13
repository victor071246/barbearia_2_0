import { Link } from 'react-router-dom';
import { useNavigationStore } from '../../store/navigationStore';
import styles from './NavBar.module.css';

export function NavBar() {
  const { currentPage, setPage } = useNavigationStore();
}
