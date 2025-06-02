import { FC } from "react";
import Sidebar from "../components/Sidebar";

const Settings: FC = () => {
  return (
    <Sidebar>
      <main className="p-6">
        <div className="max-w-[1171px] mx-auto">
          <h1 className="text-2xl font-semibold text-gray-800 mb-4">Settings</h1>
          <p className="text-gray-600">Settings panel coming soon...</p>
        </div>
      </main>
    </Sidebar>
  );
};

export default Settings;
