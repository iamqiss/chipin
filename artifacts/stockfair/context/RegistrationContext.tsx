import React, { createContext, useContext, useState } from 'react';
import type {
  Gender,
  StokvelInterest,
  SupportedLanguage,
  AppTheme,
} from '@/lib/auth/types';

// Holds registration state across the 4-step flow
interface RegistrationState {
  // Step 1
  firstName: string;
  lastName: string;
  dateOfBirth: string;
  gender: Gender | null;
  sessionKey: string; // returned from step 1 API

  // Step 2
  phone: string;
  email: string;
  password: string;

  // Step 3
  language: SupportedLanguage;

  // Step 4
  interests: StokvelInterest[];
  theme: AppTheme;
  termsAccepted: boolean;

  // OTP
  otpToken: string; // returned after OTP verification
}

const defaultState: RegistrationState = {
  firstName: '',
  lastName: '',
  dateOfBirth: '',
  gender: null,
  sessionKey: '',
  phone: '',
  email: '',
  password: '',
  language: 'en',
  interests: [],
  theme: 'obsidian',
  termsAccepted: false,
  otpToken: '',
};

interface RegistrationContextValue {
  state: RegistrationState;
  update: (patch: Partial<RegistrationState>) => void;
  reset: () => void;
}

const RegistrationContext = createContext<RegistrationContextValue | null>(null);

export function RegistrationProvider({ children }: { children: React.ReactNode }) {
  const [state, setState] = useState<RegistrationState>(defaultState);

  const update = (patch: Partial<RegistrationState>) =>
    setState(prev => ({ ...prev, ...patch }));

  const reset = () => setState(defaultState);

  return (
    <RegistrationContext.Provider value={{ state, update, reset }}>
      {children}
    </RegistrationContext.Provider>
  );
}

export function useRegistration() {
  const ctx = useContext(RegistrationContext);
  if (!ctx) throw new Error('useRegistration must be used inside RegistrationProvider');
  return ctx;
}
