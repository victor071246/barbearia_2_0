import { useState } from 'react';
//import { FontAwesomeIcon } from '@fortawesome/react-fontawesome';
//import { faXmark } from '@fortawesome/free-solid-svg-icons';
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
  const [success, setSuccess] = useState(false);

  const handleLogin = async () => {
    try {
      await login(username, password);
      setSuccess(true);
      onClose();
    } catch {
      setError('Usuário ou senha inválidos');
    }
  };

  return (
    <div
      className={`${styles.overlay} ${isOpen ? styles.overlayOpen : ''}`}
      onClick={onClose}
    >
      <div className={`${styles.panel} ${isOpen ? styles.open : ''}`}>
        <div
          className={`${styles.floatingPanel}`}
          onClick={(e) => e.stopPropagation()}
        >
          <div className={styles.title}>
            {/* <FontAwesomeIcon
              icon={faXmark}
              className={styles.closeIcon}
              //onClick={onClose}
            ></FontAwesomeIcon> */}
          </div>
          <div className={styles.form}>
            <input
              className={styles.input}
              type="text"
              placeholder="Usuário"
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
            {success && (
              <span className={styles.success}>
                Login realizado com sucesso!
              </span>
            )}
            {error && <span className={styles.error}>{error}</span>}
            <button className={styles.button} onClick={handleLogin}>
              Entrar
            </button>
          </div>
        </div>
      </div>
    </div>
  );
}
