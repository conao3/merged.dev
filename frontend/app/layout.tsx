import type { Metadata } from "next";
import "./globals.css";

export const metadata: Metadata = {
  title: "merged.dev - エンジニア向け勉強会ポータル",
  description: "エンジニアのための勉強会・イベント情報サイト",
};

export default function RootLayout({
  children,
}: {
  children: React.ReactNode;
}) {
  return (
    <html lang="ja">
      <body className="bg-gray-50">
        <header className="bg-blue-600 text-white py-4 px-8 shadow-md">
          <div className="max-w-7xl mx-auto flex items-center justify-between">
            <h1 className="text-2xl font-bold">merged.dev</h1>
            <nav className="flex gap-6">
              <a href="#" className="text-white hover:text-blue-100">
                イベント
              </a>
              <a href="#" className="text-white hover:text-blue-100">
                グループ
              </a>
              <a href="#" className="text-white hover:text-blue-100">
                ログイン
              </a>
            </nav>
          </div>
        </header>
        {children}
      </body>
    </html>
  );
}
