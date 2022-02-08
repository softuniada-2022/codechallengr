export default interface Claim {
  exp: number;
  username: string;
  perm: "guest" | "user" | "auth_author" | "admin";
}
