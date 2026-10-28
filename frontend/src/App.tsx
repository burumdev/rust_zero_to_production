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
      <Modal isOpen={isLoginModalOpen} onClose={() => setLoginModalOpen(false)}>
        <LoginModal />
      </Modal>
    </>
  );
};

export default App;
