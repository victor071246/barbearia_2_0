import { useState } from 'react';
import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
import { faXmark } from '@fortawesome/free-solid-svg-icons';
import { useAuthStore } from '../../store/authStore';
import styles from './LoginPanel.module.css';

interface LoginPanelProps {
  isOpen: boolean;
  onClose: () => void;
}

export function LoginPanel({ isOpen, onClose }: LoginPanelProps) {
  const login = useAuthStore((state) => state.login);
  const [username, setUsername] = useState('');
  const [password, setPassword] = useState('');
  const [error, setError] = useState('');

  const handleLogin = async () => {
    try {
      await login(username, password);
      onClose();
    } catch {
      setError('Usuário ou senha inválidos');
    }
  };

  return (
    <div className={`${styles.panel} ${isOpen ? styles.open : ''}`}>
      <div className={styles.title}>
        Acesso
        <FontAwesomeIcon
          icon={faXmark}
          className={styles.closeIcon}
          onClick={onClose}
        ></FontAwesomeIcon>
      </div>

      <div className={styles.form}>
        <input
          className={styles.input}
          type="text"
          value={username}
          onChange={(e) => setUsername(e.target.value)}
        ></input>

        <input
          className={styles.input}
          type="password"
          placeholder="Senha"
          value={password}
          onChange={(e) => setPassword(e.target.value)}
        ></input>

        {error && <span className={styles.error}>{error}</span>}
        <button className={styles.button} onClick={handleLogin}>
          Entrar
        </button>
      </div>
    </div>
  );
}
