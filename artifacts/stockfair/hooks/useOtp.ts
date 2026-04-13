import { useState, useRef } from 'react';
import { sendOtp, verifyOtp } from '@/lib/auth/api';
import type { OtpPurpose } from '@/lib/auth/types';

export function useOtp(phone: string, purpose: OtpPurpose) {
  const [isSending,   setIsSending]   = useState(false);
  const [isVerifying, setIsVerifying] = useState(false);
  const [error,       setError]       = useState<string | null>(null);
  const [debugOtp,    setDebugOtp]    = useState<string | null>(null);
  const [countdown,   setCountdown]   = useState(0);
  const timerRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const startCountdown = (seconds: number) => {
    setCountdown(seconds);
    timerRef.current = setInterval(() => {
      setCountdown(prev => {
        if (prev <= 1) {
          clearInterval(timerRef.current!);
          return 0;
        }
        return prev - 1;
      });
    }, 1000);
  };

  const send = async () => {
    if (countdown > 0) return;
    setIsSending(true);
    setError(null);
    const result = await sendOtp({ phone, purpose });
    setIsSending(false);
    if (!result.ok) {
      setError(result.error);
      return false;
    }
    startCountdown(result.data.expires_in_seconds);
    if (result.data.debug_otp) setDebugOtp(result.data.debug_otp);
    return true;
  };

  const verify = async (code: string): Promise<string | null> => {
    setIsVerifying(true);
    setError(null);
    const result = await verifyOtp({ phone, purpose, code });
    setIsVerifying(false);
    if (!result.ok) {
      setError(result.error);
      return null;
    }
    return result.data.otp_token;
  };

  return {
    send,
    verify,
    isSending,
    isVerifying,
    error,
    debugOtp,
    countdown,
    canResend: countdown === 0 && !isSending,
  };
}
