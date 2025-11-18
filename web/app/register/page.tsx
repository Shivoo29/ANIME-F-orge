"use client";

import { useState, FormEvent } from "react";
import { useRouter } from "next/navigation";
import Link from "next/link";
import Button from "@/components/Button";
import { useAuthStore } from "@/store/authStore";
import { authAPI } from "@/lib/api";

export default function RegisterPage() {
  const router = useRouter();
  const { login } = useAuthStore();
  const [username, setUsername] = useState("");
  const [email, setEmail] = useState("");
  const [password, setPassword] = useState("");
  const [confirmPassword, setConfirmPassword] = useState("");
  const [error, setError] = useState("");
  const [isLoading, setIsLoading] = useState(false);

  const handleSubmit = async (e: FormEvent) => {
    e.preventDefault();
    setError("");

    // Validation
    if (!username || !email || !password || !confirmPassword) {
      setError("Please fill in all fields");
      return;
    }

    if (username.length < 3) {
      setError("Username must be at least 3 characters long");
      return;
    }

    if (!email.includes("@")) {
      setError("Please enter a valid email address");
      return;
    }

    if (password.length < 8) {
      setError("Password must be at least 8 characters long");
      return;
    }

    if (password !== confirmPassword) {
      setError("Passwords do not match");
      return;
    }

    setIsLoading(true);

    try {
      const response = await authAPI.register(username, email, password);
      const { user, token } = response.data;
      login(user, token);
      router.push("/dashboard");
    } catch (err: any) {
      setError(
        err.response?.data?.message ||
          "Failed to create account. Please try again."
      );
    } finally {
      setIsLoading(false);
    }
  };

  return (
    <div className="min-h-[80vh] flex items-center justify-center py-12">
      <div className="brutal-container max-w-md w-full">
        <div className="brutal-card p-8">
          <h1 className="text-4xl mb-2 text-center">SIGN UP</h1>
          <p className="text-center text-gray-600 mb-8">
            Start creating amazing animations today
          </p>

          {error && (
            <div className="mb-6 p-4 border-brutal border-dark bg-red-100 text-red-700">
              <p className="font-bold">⚠️ {error}</p>
            </div>
          )}

          <form onSubmit={handleSubmit} className="space-y-6">
            <div>
              <label htmlFor="username" className="block font-bold mb-2 uppercase">
                Username
              </label>
              <input
                id="username"
                type="text"
                value={username}
                onChange={(e) => setUsername(e.target.value)}
                className="brutal-input w-full"
                placeholder="coolcreator123"
                disabled={isLoading}
              />
            </div>

            <div>
              <label htmlFor="email" className="block font-bold mb-2 uppercase">
                Email Address
              </label>
              <input
                id="email"
                type="email"
                value={email}
                onChange={(e) => setEmail(e.target.value)}
                className="brutal-input w-full"
                placeholder="your@email.com"
                disabled={isLoading}
              />
            </div>

            <div>
              <label htmlFor="password" className="block font-bold mb-2 uppercase">
                Password
              </label>
              <input
                id="password"
                type="password"
                value={password}
                onChange={(e) => setPassword(e.target.value)}
                className="brutal-input w-full"
                placeholder="••••••••"
                disabled={isLoading}
              />
              <p className="text-sm text-gray-600 mt-1">
                At least 8 characters
              </p>
            </div>

            <div>
              <label
                htmlFor="confirmPassword"
                className="block font-bold mb-2 uppercase"
              >
                Confirm Password
              </label>
              <input
                id="confirmPassword"
                type="password"
                value={confirmPassword}
                onChange={(e) => setConfirmPassword(e.target.value)}
                className="brutal-input w-full"
                placeholder="••••••••"
                disabled={isLoading}
              />
            </div>

            <Button
              type="submit"
              variant="primary"
              size="lg"
              className="w-full"
              disabled={isLoading}
            >
              {isLoading ? "CREATING ACCOUNT..." : "CREATE ACCOUNT"}
            </Button>
          </form>

          <div className="mt-6 text-center">
            <p className="text-gray-600">
              Already have an account?{" "}
              <Link
                href="/login"
                className="font-bold text-primary hover:underline"
              >
                Login
              </Link>
            </p>
          </div>

          <div className="mt-6 pt-6 border-t-brutal border-dark text-center text-sm text-gray-600">
            <p>
              By signing up, you agree to our{" "}
              <Link href="#terms" className="font-bold hover:text-primary">
                Terms of Service
              </Link>{" "}
              and{" "}
              <Link href="#privacy" className="font-bold hover:text-primary">
                Privacy Policy
              </Link>
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}
