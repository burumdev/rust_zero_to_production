import { useEffect, useRef } from 'react';
import './TextInput.css';

interface TextInputProps {
  placeholder?: string;
  is_focused?: boolean;
  onChange: (e: any) => void;
  value: string;
}

const TextInput: React.FC<TextInputProps> = ({
  placeholder,
  is_focused,
  onChange,
  value,
}) => {
  const inputRef = useRef<HTMLInputElement>(null);

  useEffect(() => {
    if (is_focused && inputRef.current) {
      inputRef.current.focus();
    }
  }, [is_focused]);

  return (
    <div className="container-textinput w-full h-full">
      <input
        ref={inputRef}
        className="step-box bg-yellow w-full h-full px-3 placeholder-yellow-100"
        placeholder={placeholder || undefined}
        onChange={(e) => onChange(e)}
        value={value}
      />
    </div>
  );
};

export default TextInput;
