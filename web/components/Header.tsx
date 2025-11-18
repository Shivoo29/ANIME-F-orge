"use client";

import Link from "next/link";
import Button from "./Button";
import { useAuthStore } from "@/store/authStore";

export default function Header() {
  const { user, logout } = useAuthStore();

  return (
    <header className="border-b-brutal border-dark bg-white shadow-brutal-sm">
      <div className="brutal-container">
        <div className="flex items-center justify-between py-4">
          <Link href="/" className="flex items-center gap-2">
            <div className="w-12 h-12 bg-primary border-brutal border-dark flex items-center justify-center shadow-brutal-sm">
              <span className="text-2xl font-bold text-white">AF</span>
            </div>
            <span className="text-2xl font-bold uppercase">AnimaForge</span>
          </Link>

          <nav className="hidden md:flex items-center gap-6">
            <Link
              href="/browse"
              className="font-bold uppercase hover:text-primary transition-colors"
            >
              Browse
            </Link>
            <Link
              href="#features"
              className="font-bold uppercase hover:text-primary transition-colors"
            >
              Features
            </Link>
            <Link
              href="#how-it-works"
              className="font-bold uppercase hover:text-primary transition-colors"
            >
              How It Works
            </Link>
          </nav>

          <div className="flex items-center gap-3">
            {user ? (
              <>
                <Link href="/dashboard">
                  <Button variant="outline" size="sm">
                    Dashboard
                  </Button>
                </Link>
                <Button variant="primary" size="sm" onClick={logout}>
                  Logout
                </Button>
              </>
            ) : (
              <>
                <Link href="/login">
                  <Button variant="outline" size="sm">
                    Login
                  </Button>
                </Link>
                <Link href="/register">
                  <Button variant="primary" size="sm">
                    Sign Up
                  </Button>
                </Link>
              </>
            )}
          </div>
        </div>
      </div>
    </header>
  );
}
