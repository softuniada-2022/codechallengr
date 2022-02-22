import type Score from "@/models/Score";
import useSWRV, { mutate } from "swrv";

export function prefetchScores() {
  const url = "/api/score";
  const dataPromise = fetch(url).then((res) => res.json());
  mutate(url, dataPromise);
  return dataPromise;
}

export function useScores() {
  const url = "/api/score";
  const { data, error } = useSWRV<Score[]>(url);
  return { data, error };
}
