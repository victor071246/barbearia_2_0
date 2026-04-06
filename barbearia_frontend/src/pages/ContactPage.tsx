import styles from './ContactPage.module.css';

export function ContactPage() {
  return (
    <div className={styles.container}>
      <div className={styles.mapa_container}>
        <div className={styles.mapa}>
          <iframe
            src="https://www.google.com/maps/embed?pb=!1m18!1m12!1m3!1d3700.0!2d-48.17!3d-21.79!2m3!1f0!2f0!3f0!3m2!1i1024!2i768!4f13.1!3m3!1m2!1s0x0%3A0x0!2zMjHCsDQ3JzI0LjAiUyA0OMKwMTAnMTIuMCJX!5e0!3m2!1spt-BR!2sbr!4v1234567890"
            width="100%"
            height="100%"
            style={{ border: 0 }}
            allowFullScreen
            loading="lazy"
            referrerPolicy="no-referrer-when-downgrade"
          />
        </div>
        <div className={styles.info}>
          <div className={styles.item}>
            <span className={styles.label}>Endereço</span>
            <span className={styles.valor}>
              Rua Exemplo, 123 - Araraquara, São Paulo
            </span>
          </div>
        </div>
      </div>
    </div>
  );
}
