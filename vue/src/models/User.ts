export default interface User {
  u_name: string;
  u_email: string;
  u_score: number;
  u_permission: "guest" | "user" | "auth_author" | "admin";
  u_created_at: string;
  u_updated_at: string;
}
