import { useState } from 'react';
import './App.css';

import Header from './components/Header';
import Modal from './components/common/Modal';
import LoginModal from './components/LoginModal';

const App = () => {
  const [isLoginModalOpen, setLoginModalOpen] = useState(true);

  return (
    <>
      <Header />
      <div className="flex flex-col items-center justify-center">
        <img className="-mt-[20px]" src="/images/logo.svg" />
        <p className="w-[640px] italic text-center text-sm">
          A place where dreams turn into nested, <br /> self-referential
          nightmares.
          <br />
          But you don't remember them when you wake up.
        </p>
      </div>
      <Modal isOpen={isLoginModalOpen} onClose={() => setLoginModalOpen(false)}>
        <LoginModal />
      </Modal>
    </>
  );
};

export default App;
