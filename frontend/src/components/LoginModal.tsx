import Button from './common/Button';
import TextInput from './common/TextInput';

const LoginModal = () => {
  return (
    <div className="max-w-lg">
      <h1 className="text-center text-lg">WELCOME TO ZERO2PROD!</h1>
      <p className="mt-3">
        Enter your email and name to join our newsletter and gain access to our
        exclusive chat rooms.
      </p>
      <div className="mt-4 h-forminput">
        <TextInput placeholder="E-mail*" is_focused />
      </div>
      <div className="mt-4 h-forminput">
        <TextInput placeholder="Name*" />
      </div>
      <div className="mt-4 h-forminput flex justify-end">
        <Button
          onClick={() => {}}
          text="JOIN"
          backgroundClass="bg-highlight"
          dropShadowColor="highlight-darker"
        />
      </div>
    </div>
  );
};

export default LoginModal;
