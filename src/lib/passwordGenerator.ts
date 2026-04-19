export function generatePassword(length: number): string {
  if (length < 12) {
    throw new Error("Długość hasła musi wynosić co najmniej 8");
  }

  const lowercase = "abcdefghijklmnopqrstuvwxyz";
  const uppercase = "ABCDEFGHIJKLMNOPQRSTUVWXYZ";
  const digits = "0123456789";
  const special = "!@#$%^&*()-_=+[]{};:,.<>?/|";

  const all = lowercase + uppercase + digits + special;

  const getRandom = (chars: string) =>
    chars[Math.floor(Math.random() * chars.length)];

  const password: string[] = [];

  // gwarantowane wymagane znaki
  password.push(getRandom(lowercase));
  password.push(getRandom(uppercase));
  password.push(getRandom(digits));
  password.push(getRandom(special));

  // reszta znaków
  while (password.length < length) {
    const next = getRandom(all);

    // blokada dwóch identycznych znaków obok siebie
    if (next !== password[password.length - 1]) {
      password.push(next);
    }
  }

  for (let i = password.length - 1; i > 0; i--) {
    const j = Math.floor(Math.random() * (i + 1));
    [password[i], password[j]] = [password[j], password[i]];
  }

  return password.join("");
}

const password = generatePassword(12);
console.log(password);
