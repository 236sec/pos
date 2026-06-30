"use client";

import { useState, useEffect, useCallback } from "react";
import type { LoginRequest, LoginResponse, User } from "@pos/shared/types";
import { ApiClient } from "@pos/shared/api";

const TOKEN_KEY = "pos_auth_token";

export function useAuth(apiUrl: string) {
  const [user, setUser] = useState<User | null>(null);
  const [loading, setLoading] = useState(true);

  const getToken = useCallback(() => {
    if (typeof window === "undefined") return null;
    return localStorage.getItem(TOKEN_KEY);
  }, []);

  const client = useCallback(() => {
    return new ApiClient(apiUrl, getToken);
  }, [apiUrl, getToken]);

  const login = useCallback(
    async (req: LoginRequest): Promise<LoginResponse> => {
      const c = new ApiClient(apiUrl, () => null);
      const res = await c.post<LoginResponse>("/auth/login", req);
      localStorage.setItem(TOKEN_KEY, res.token);
      setUser(res.user);
      return res;
    },
    [apiUrl],
  );

  const logout = useCallback(async () => {
    const c = client();
    try {
      await c.post<void>("/auth/logout");
    } catch {
      // ignore network errors during logout
    }
    localStorage.removeItem(TOKEN_KEY);
    setUser(null);
  }, [client]);

  const fetchMe = useCallback(async () => {
    const token = getToken();
    if (!token) {
      setLoading(false);
      return;
    }
    try {
      const c = client();
      const u = await c.get<User>("/auth/me");
      setUser(u);
    } catch {
      localStorage.removeItem(TOKEN_KEY);
      setUser(null);
    } finally {
      setLoading(false);
    }
  }, [getToken, client]);

  useEffect(() => {
    fetchMe();
  }, [fetchMe]);

  return { user, loading, login, logout };
}
