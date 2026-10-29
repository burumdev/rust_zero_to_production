interface ButtonProps {
  onClick: () => void;
  backgroundClass?: string;
  dropShadowColor?: string;
  text?: string;
}

const Button: React.FC<ButtonProps> = ({
  onClick,
  text,
  backgroundClass = 'bg-yellow',
  dropShadowColor = 'yellow',
}) => {
  return (
    <div
      className="h-full"
      style={{
        filter: `drop-shadow(4px 4px 0 var(--color-${dropShadowColor}))`,
      }}
    >
      <button
        className={`step-box flex justify-center items-center h-full px-3 cursor-pointer ${backgroundClass}`}
        onClick={() => onClick()}
      >
        {text}
      </button>
    </div>
  );
};

export default Button;
