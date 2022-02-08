import type User from "@/models/User";
import useSWRV, { mutate } from "swrv";
import { logIn } from "@/api/login";

export function prefetchUsers() {
  const url = "/api/user";
  mutate(
    url,
    fetch(url).then((r) => r.json())
  );
}

export function prefetchUser(username: string, data?: User) {
  const url = `/api/user/${username}`;
  mutate(url, data ?? fetch(url).then((r) => r.json()));
}

export function useUsers() {
  const { data, error } = useSWRV<User[]>("/api/user");
  data.value?.forEach((user) => prefetchUser(user.u_name));
  return { data, error };
}

export function useUser(username: string) {
  const { data, error } = useSWRV<User>(`/api/user/${username}`);
  return { data, error };
}

export async function signUp(
  username: string,
  email: string,
  password: string,
  logInAfterSignUp = true
): Promise<void> {
  const url = "/api/user";
  const body = JSON.stringify({
    u_name: username,
    u_email: email,
    u_password: password,
  });
  const resp = await fetch(url, {
    method: "POST",
    body,
    headers: {
      "Content-Type": "application/json",
    },
  });
  if (!resp.ok) {
    throw new Error(await resp.text());
  }
  if (logInAfterSignUp) {
    await logIn(username, password);
  }
}