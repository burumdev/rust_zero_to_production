import Button from '@components/Button';
import TextInput from '@components/TextInput';

import { useUserNewsletterRegister } from '@user/hooks/use_user';
import type { UserNewsletterRegisterRequest } from '@user/user_types';
import { useState } from 'react';

const LoginModal = () => {
  const newsletterRegister = useUserNewsletterRegister();
  const [registerFormData, setRegisterFormData] =
    useState<UserNewsletterRegisterRequest>({
      email: '',
      name: '',
    });

  const handleFormDataChange = (field: string, value: string) => {
    setRegisterFormData({
      ...registerFormData,
      [field]: value,
    });
  };

  const handleSubmit = () => {
    newsletterRegister.mutate(registerFormData, {
      onSuccess: () => console.log('User registration success.'),
    });
  };

  return (
    <div className="max-w-lg">
      <h1 className="text-center text-lg">WELCOME TO ZERO2PROD!</h1>
      <p className="mt-3">
        Enter your email and name to join our newsletter and gain access to our
        exclusive chat rooms.
      </p>
      <div className="mt-4 h-forminput">
        <TextInput
          placeholder="E-mail*"
          is_focused
          onChange={(e: any) => handleFormDataChange('email', e.target.value)}
          value={registerFormData.email}
        />
      </div>
      <div className="mt-4 h-forminput">
        <TextInput
          placeholder="Name*"
          onChange={(e: any) => handleFormDataChange('name', e.target.value)}
          value={registerFormData.name}
        />
      </div>
      <div className="mt-4 h-forminput flex justify-end">
        <Button
          onClick={handleSubmit}
          text="JOIN"
          backgroundClass="bg-highlight"
          dropShadowColor="highlight-darker"
        />
      </div>
    </div>
  );
};

export default LoginModal;
