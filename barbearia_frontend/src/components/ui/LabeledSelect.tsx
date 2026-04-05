import styles from './LabeledInput.module.css';

interface LabeledSelectProps extends React.SelectHTMLAttributes<HTMLSelectElement> {
  label: string;
  children: React.ReactNode;
}

export function LabeledSelect({
  label,
  className,
  children,
  ...props
}: LabeledSelectProps) {
  return (
    <div className={styles.wrapper}>
      <select className={`${styles.input} ${className ?? ''}}`} {...props}>
        {children}
      </select>
      <span className={styles.label}>{label}</span>
    </div>
  );
}
