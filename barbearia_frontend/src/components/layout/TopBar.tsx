import { useEffect, useState } from 'react';
import { api } from '../../api/client';
import styles from './TopBar.module.css';
import { useAuthStore, type User } from '../../store/authStore';

export function TopBar() {
  const user = useAuthStore((state) => state.user);
  const checkAuth = useAuthStore((state) => state.checkAuth);
  const logout = useAuthStore((state) => state.logout);

  useEffect(() => {
    checkAuth();
  }, [checkAuth]);

  return (
    <div className={styles.topbar}>
      <div className={styles.logo}>
        <img src="../assets/logo.png"></img>
      </div>
      <div className={styles.auth}>
        {user ? (
          <>
            <span className={styles.username}>Olá, {user.username}</span>
            <button className={styles.btn} onClick={logout}>
              Sair
            </button>
          </>
        ) : (
          <button className={styles.btn}>Entrar</button>
        )}
        ;
      </div>
    </div>
  );
}
