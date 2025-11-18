import Link from "next/link";

export default function Footer() {
  const currentYear = new Date().getFullYear();

  return (
    <footer className="border-t-brutal border-dark bg-secondary text-white mt-20">
      <div className="brutal-container py-12">
        <div className="grid grid-cols-1 md:grid-cols-4 gap-8">
          <div>
            <h4 className="text-xl font-bold uppercase mb-4">AnimaForge</h4>
            <p className="text-sm">
              Transform words into stunning animations. No code required.
            </p>
          </div>

          <div>
            <h4 className="text-lg font-bold uppercase mb-4">Product</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <Link href="/browse" className="hover:text-accent transition-colors">
                  Browse Animations
                </Link>
              </li>
              <li>
                <Link href="#features" className="hover:text-accent transition-colors">
                  Features
                </Link>
              </li>
              <li>
                <Link href="#pricing" className="hover:text-accent transition-colors">
                  Pricing
                </Link>
              </li>
            </ul>
          </div>

          <div>
            <h4 className="text-lg font-bold uppercase mb-4">Resources</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <Link href="#docs" className="hover:text-accent transition-colors">
                  Documentation
                </Link>
              </li>
              <li>
                <Link href="#api" className="hover:text-accent transition-colors">
                  API Reference
                </Link>
              </li>
              <li>
                <Link href="#support" className="hover:text-accent transition-colors">
                  Support
                </Link>
              </li>
            </ul>
          </div>

          <div>
            <h4 className="text-lg font-bold uppercase mb-4">Legal</h4>
            <ul className="space-y-2 text-sm">
              <li>
                <Link href="#privacy" className="hover:text-accent transition-colors">
                  Privacy Policy
                </Link>
              </li>
              <li>
                <Link href="#terms" className="hover:text-accent transition-colors">
                  Terms of Service
                </Link>
              </li>
            </ul>
          </div>
        </div>

        <div className="mt-8 pt-8 border-t-brutal border-white/20 text-center text-sm">
          <p>&copy; {currentYear} AnimaForge. All rights reserved.</p>
        </div>
      </div>
    </footer>
  );
}
