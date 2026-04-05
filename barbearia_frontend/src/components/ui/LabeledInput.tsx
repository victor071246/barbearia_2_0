import styles from './LabeledInput.module.css';

interface LabeledInputProps extends React.InputHTMLAttributes<HTMLInputElement> {
  label: string;
}

export function LabeledInput({
  label,
  className,
  ...props
}: LabeledInputProps) {
  return (
    <div className={styles.wrapper}>
      <input
        className={`${styles.input} ${className ?? ''} `}
        {...props}
      ></input>
      <span className={styles.label}>{label}</span>
    </div>
  );
}
