"use client";

import { useRouter } from "next/navigation";
import { useState } from "react";
import { LoginForm } from "@pos/shared/ui";
import { useAuth } from "@pos/shared/hooks";
import { env } from "@/env";

export default function LoginPage() {
  const router = useRouter();
  const { login } = useAuth(env.NEXT_PUBLIC_API_URL);
  const [error, setError] = useState<string | null>(null);
  const [loading, setLoading] = useState(false);

  const handleLogin = async (username: string, password: string) => {
    setError(null);
    setLoading(true);
    try {
      await login({ username, password });
      router.push("/");
    } catch (e: any) {
      setError(e?.body?.message || e?.message || "Login failed");
    } finally {
      setLoading(false);
    }
  };

  return (
    <main className="flex min-h-screen items-center justify-center p-8">
      <LoginForm onSubmit={handleLogin} error={error} loading={loading} />
    </main>
  );
}
