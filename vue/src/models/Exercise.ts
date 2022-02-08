export default interface Exercise {
  ex_id: number;
  u_id: number;
  ex_name: string;
  ex_description: string;
  ex_input: string;
  ex_difficulty: number;
  ex_likes: number;
  ex_liked_by_me?: boolean;
  ex_solved_by_me?: boolean;
  ex_created_at: string;
  ex_updated_at: string;
}

export interface CreateExercise {
  ex_name: string;
  ex_description: string;
  ex_input: string;
  ex_answer: string;
  ex_difficulty: number;
}
