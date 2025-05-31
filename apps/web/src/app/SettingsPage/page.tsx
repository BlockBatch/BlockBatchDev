import { FC } from "react";
import SettingsClient from "./components/SettingsClient";

// In Next.js app directory, page components are server components by default
const Settings: FC = () => {

  return (
    <main className="min-h-screen bg-gray-50 py-8">
      <div className="max-w-7xl mx-auto px-4 sm:px-6 lg:px-8">
        <div className="max-w-6xl mx-auto">
          <h1 className="text-2xl font-semibold mb-6">Settings</h1>
          <p className="text-gray-600 mb-8">Manage your account, settings and preferences.</p>
          
          {/* Tabs */}
          <div className="border-b border-gray-200">
            <nav className="-mb-px flex space-x-8">
              <a href="#" className="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">
                Profile
              </a>
              <a href="#" className="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">
                Notifications
              </a>
              <a href="#" className="border-transparent text-gray-500 hover:border-gray-300 hover:text-gray-700 whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium">
                Wallets
              </a>
              <a href="#" className="border-black text-black whitespace-nowrap border-b-2 py-4 px-1 text-sm font-medium" aria-current="page">
                API Keys
              </a>
            </nav>
          </div>
          
          {/* Use client-only component for the interactive part */}
          <SettingsClient />
        </div>
      </div>
    </main>
  );
};

export default Settings;
