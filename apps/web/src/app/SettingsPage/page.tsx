"use client";

import { useState } from "react";

const TABS = [
  { label: "Profile" },
  { label: "Notifications" },
  { label: "Wallets" },
  { label: "API Keys" },
];

export default function Settings() {
  const [activeTab, setActiveTab] = useState("Profile");
  const [twoFactor, setTwoFactor] = useState(false);
  const [sessionTimeout, setSessionTimeout] = useState(true);

  return (
    <main className="min-h-screen bg-gray-50 flex flex-col items-center py-12">
      <div className="w-full max-w-2xl">
        <h1
          className="mb-2"
          style={{
            fontFamily: 'Geist, sans-serif',
            fontWeight: 400,
            fontSize: '36px',
            lineHeight: '40px',
            color: '#3F3F46',
            letterSpacing: 0,
            verticalAlign: 'middle',
          }}
        >
          Settings
        </h1>
        <p
          className="mb-6"
          style={{
            fontFamily: 'Geist, sans-serif',
            fontWeight: 400,
            fontSize: '18px',
            lineHeight: '28px',
            color: '#71717A',
            letterSpacing: 0,
            verticalAlign: 'middle',
          }}
        >
          Manage your account settings and preferences
        </p>
        <div className="mb-6">
          <nav
            className="flex bg-gray-100 rounded-lg p-1 w-fit gap-1"
            aria-label="Tabs"
          >
            {TABS.map((tab) => (
              <button
                key={tab.label}
                onClick={() => setActiveTab(tab.label)}
                style={{
                  fontFamily: 'Geist, sans-serif',
                  fontWeight: 500,
                  fontSize: '14px',
                  lineHeight: '20px',
                  letterSpacing: 0,
                  textAlign: 'center',
                  verticalAlign: 'middle',
                  color: activeTab === tab.label ? '#09090B' : '#71717A',
                }}
                className={`px-5 py-2 rounded-md transition-all focus:outline-none
                  ${
                    activeTab === tab.label
                      ? "bg-white shadow font-semibold"
                      : "bg-transparent hover:text-black"
                  }
                `}
              >
                {tab.label}
              </button>
            ))}
          </nav>
        </div>
        <div>
          {activeTab === "Profile" && (
            <div className="bg-white rounded-lg shadow p-6">
              {/* Profile Form */}
              <form className="space-y-8">
                {/* Personal Information */}
                <div>
                  <h2 className="text-lg font-semibold mb-1" style={{fontFamily: 'Geist, sans-serif', color: '#18181B'}}>Profile</h2>
                  <p className="text-sm mb-4" style={{fontFamily: 'Geist, sans-serif', color: '#71717A'}}>Manage your personal information and company details</p>
                  <h3 className="mb-2" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '18px', lineHeight: '28px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Personal Information</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Full Name</label>
                      <input type="text" placeholder="John Doe" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Email</label>
                      <input type="email" placeholder="john@example.com" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Phone Number</label>
                      <input type="tel" placeholder="+1 (555) 123-4567" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Timezone</label>
                      <input type="text" placeholder="America/New_York" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                    </div>
                  </div>
                </div>
                {/* Company Information */}
                <div>
                  <h3 className="mb-2" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '18px', lineHeight: '28px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Company Information</h3>
                  <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Company Name</label>
                      <input type="text" placeholder="Acme Inc." className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                    </div>
                    <div>
                      <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Website</label>
                      <input type="url" placeholder="https://acme.com" className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}} />
                    </div>
                  </div>
                  <div>
                    <label className="block mb-1" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Address</label>
                    <textarea
                      placeholder="123 Main St, Suite 100, New York, NY 10001"
                      rows={3}
                      className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300 resize-none"
                      style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '100%', letterSpacing: 0, verticalAlign: 'middle'}}
                    />
                  </div>
                </div>
                {/* Security */}
                <div>
                  <h3 className="mb-2" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '18px', lineHeight: '28px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Security</h3>
                  <div className="flex flex-col gap-4">
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Two-Factor Authentication</label>
                        <span className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '20px', letterSpacing: 0, verticalAlign: 'middle', color: '#71717A'}}>Add an extra layer of security to your account</span>
                      </div>
                      {/* Switch para Two-Factor Authentication */}
                      <button
                        type="button"
                        aria-pressed={twoFactor}
                        onClick={() => setTwoFactor((v) => !v)}
                        className={`w-11 h-6 rounded-full transition-colors duration-200 flex items-center focus:outline-none shadow-sm
                          ${twoFactor ? 'bg-[#18181B]' : 'bg-[#E4E4E7]'}
                        `}
                        style={{ minWidth: 44 }}
                      >
                        <span
                          className={`inline-block w-5 h-5 bg-white rounded-full shadow transform transition-transform duration-200
                            ${twoFactor ? 'translate-x-5' : 'translate-x-1'}
                          `}
                        />
                      </button>
                    </div>
                    <div className="flex items-center justify-between">
                      <div>
                        <label className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 500, fontSize: '14px', lineHeight: '14px', letterSpacing: 0, verticalAlign: 'middle', color: '#09090B'}}>Session Timeout</label>
                        <span className="block" style={{fontFamily: 'Geist, sans-serif', fontWeight: 400, fontSize: '14px', lineHeight: '20px', letterSpacing: 0, verticalAlign: 'middle', color: '#71717A'}}>Automatically log out after period of inactivity</span>
                      </div>
                      {/* Switch para Session Timeout */}
                      <button
                        type="button"
                        aria-pressed={sessionTimeout}
                        onClick={() => setSessionTimeout((v) => !v)}
                        className={`w-11 h-6 rounded-full transition-colors duration-200 flex items-center focus:outline-none shadow-sm
                          ${sessionTimeout ? 'bg-[#18181B]' : 'bg-[#E4E4E7]'}
                        `}
                        style={{ minWidth: 44 }}
                      >
                        <span
                          className={`inline-block w-5 h-5 bg-white rounded-full shadow transform transition-transform duration-200
                            ${sessionTimeout ? 'translate-x-5' : 'translate-x-1'}
                          `}
                        />
                      </button>
                    </div>
                  </div>
                </div>
                <div className="flex justify-start">
                  <button
                    type="submit"
                    style={{
                      background: '#18181B',
                      fontFamily: 'Geist, sans-serif',
                      fontWeight: 500,
                      fontSize: '14px',
                      lineHeight: '20px',
                      letterSpacing: 0,
                      textAlign: 'center',
                      verticalAlign: 'middle',
                      color: '#FAFAFA',
                    }}
                    className="px-6 py-2 rounded-md transition hover:bg-gray-900"
                  >
                    Save Changes
                  </button>
                </div>
              </form>
            </div>
          )}
          {activeTab === "Notifications" && (
            <div className="bg-white rounded-lg shadow p-6 flex items-center justify-center min-h-[200px]">
              <span className="text-gray-400 text-lg font-medium">Coming soon...</span>
            </div>
          )}
          {activeTab !== "Profile" && activeTab !== "Notifications" && (
            <div className="bg-white rounded-lg shadow p-6 flex items-center justify-center min-h-[200px]">
              <span className="text-gray-400 text-lg font-medium">Coming soon...</span>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
