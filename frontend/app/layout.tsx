import type { Metadata } from "next";
import "./globals.css";
import { Providers } from "@/providers/providers";
import { geistMono } from "@/fonts";
import { Toaster } from "@/components/ui/sonner";
import { Loader } from "@/components/ui/loader";
import Footer from "@/components/layout/footer";

export const metadata: Metadata = {
  title: "CasinoJam",
  description:
    "CasinoJam - Demo Terminal App for interacting with the SAGE game engine by Ajuna",
  metadataBase: new URL(
    process.env.NODE_ENV === "development"
      ? "http://localhost:3000"
      : "https://casinojam.netlify.app/"
  ),
  openGraph: {
    title: "CasinoJam",
    description:
      "CasinoJam - Demo Terminal App for interacting with the SAGE game engine by Ajuna",
    url: "https://casinojam.netlify.app/",
    siteName: "CasinoJam",
  },
};

export default function RootLayout({
  children,
}: Readonly<{
  children: React.ReactNode;
}>) {
  return (
    <html lang="en" suppressHydrationWarning>
      <body className={`${geistMono.className} antialiased`}>
        <Providers>
          {children}
          <Footer />
          <Toaster position="bottom-center" icons={{ loading: <Loader /> }} />
        </Providers>
      </body>
    </html>
  );
}
