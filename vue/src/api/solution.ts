import type Exercise from "@/models/Exercise";
import type Solution from "@/models/Solution";
import useSWRV, { mutate } from "swrv";

export function prefetchSolutions(exerciseId: number) {
  const url = `/api/solution?exercise=${exerciseId}`;
  const dataPromise = fetch(url).then((res) => res.json());
  mutate(url, dataPromise);
  return dataPromise;
}

export function useSolutions(exerciseId: number) {
  const url = `/api/solution?exercise=${exerciseId}`;
  const { data, error } = useSWRV<Solution[]>(url);
  return { data, error };
}

export async function submitSolution(exercise: Exercise, solution: string) {
  const url = `/api/solution`;
  const body = JSON.stringify({
    ex_id: exercise.ex_id,
    s_answer: solution,
  });
  const resp = await fetch(url, {
    method: "POST",
    headers: {
      "Content-Type": "application/json",
    },
    body,
  });
  if (!resp.ok) {
    throw new Error(await resp.text());
  }
  prefetchSolutions(exercise.ex_id);
}
