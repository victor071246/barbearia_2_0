import { useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSignInAlt, faSignOutAlt } from '@fortawesome/free-solid-svg-icons';
import { useAuthStore, type User } from '../../store/authStore';
import styles from './TopBar.module.css';
import logo from './../../assets/logo.png';

export function TopBar() {
  const user = useAuthStore((state) => state.user);
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const checkAuth = useAuthStore((state) => state.checkAuth);
  const logout = useAuthStore((state) => state.logout);

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  return (
    <div className={styles.topbar}>
      <div className={styles.logo}>
        <img src={logo}></img>
      </div>
      <div className={styles.auth}>
        {isAuthenticated ? (
          <FontAwesomeIcon
            className={styles.icon}
            icon={faSignOutAlt}
          ></FontAwesomeIcon>
        ) : (
          <FontAwesomeIcon
            icon={faSignInAlt}
            className={styles.icon}
          ></FontAwesomeIcon>
        )}
      </div>
    </div>
  );
}
