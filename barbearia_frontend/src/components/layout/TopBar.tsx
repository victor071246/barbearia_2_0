import { useState, useEffect } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faSignInAlt, faSignOutAlt } from '@fortawesome/free-solid-svg-icons';
import { useAuthStore } from '../../store/authStore';
import { LoginPanel } from './LoginPanel';
import styles from './TopBar.module.css';
import logo from './../../assets/logo.png';

export function TopBar() {
  const isAuthenticated = useAuthStore((state) => state.isAuthenticated);
  const checkAuth = useAuthStore((state) => state.checkAuth);
  const logout = useAuthStore((state) => state.logout);
  const [loginOpen, setLoginOpen] = useState(false);

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
            onClick={() => logout()}
          ></FontAwesomeIcon>
        ) : (
          <FontAwesomeIcon
            className={styles.icon}
            icon={faSignInAlt}
            onClick={() => setLoginOpen(true)}
          ></FontAwesomeIcon>
        )}
      </div>

      <LoginPanel
        isOpen={loginOpen}
        onClose={() => setLoginOpen(false)}
      ></LoginPanel>
    </div>
  );
}
