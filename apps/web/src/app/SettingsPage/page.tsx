"use client";

import { useState } from "react";
import { Copy, Trash2, Plus } from "lucide-react";

const TABS = [
  { label: "Profile" },
  { label: "Notifications" },
  { label: "Wallets" },
  { label: "API Keys" },
];

export default function Settings() {
  const [activeTab, setActiveTab] = useState("Profile");

  return (
    <main className="min-h-screen bg-gray-50 flex flex-col items-center py-12">
      <div className="w-full max-w-4xl mx-auto px-4 sm:px-6 lg:px-8">
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
        <div
          className="bg-white rounded-lg shadow p-8 border border-gray-200"
          style={{ width: '920.56px', margin: '0 auto' }}
        >
          {activeTab === "Wallets" ? (
            <>
              <h2
                style={{
                  fontFamily: 'Geist',
                  fontWeight: 600,
                  fontSize: '24px',
                  lineHeight: '24px',
                  letterSpacing: '-0.6px',
                  verticalAlign: 'middle',
                  color: '#09090B',
                }}
                className="mb-1"
              >
                Blockchain Wallets
              </h2>
              <p
                style={{
                  fontFamily: 'Geist',
                  fontWeight: 400,
                  fontSize: '14px',
                  lineHeight: '20px',
                  letterSpacing: '0%',
                  verticalAlign: 'middle',
                  color: '#71717A',
                }}
                className="mb-6"
              >
                Manage your connected blockchain wallets for batch payments
              </p>
              <div className="flex justify-between items-center mb-4">
                <h3
                  style={{
                    fontFamily: 'Geist',
                    fontWeight: 500,
                    fontSize: '18px',
                    lineHeight: '28px',
                    letterSpacing: '0%',
                    verticalAlign: 'middle',
                    color: '#09090B',
                  }}
                >Connected Wallets</h3>
                <button
                  style={{
                    fontFamily: 'Geist',
                    fontWeight: 500,
                    fontSize: '14px',
                    lineHeight: '20px',
                    letterSpacing: '0%',
                    textAlign: 'center',
                    verticalAlign: 'middle',
                    color: '#FAFAFA',
                    backgroundColor: '#18181B',
                  }}
                  className="px-4 py-2 rounded-md text-sm font-medium flex items-center gap-1 hover:opacity-90 transition-opacity"
                >
                  <Plus size={16} />
                  Add Wallet
                </button>
              </div>
              <div className="overflow-x-auto mb-8">
                <table className="w-full text-sm border-separate border-spacing-0 border border-gray-200 rounded-md overflow-hidden">
                  <thead>
                    <tr>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Name
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Wallet Address
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Network
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Status
                      </th>
                      <th
                        className="text-left px-4 py-2"
                        style={{
                          fontFamily: 'Geist',
                          fontWeight: 500,
                          fontSize: '14px',
                          lineHeight: '20px',
                          letterSpacing: '0%',
                          verticalAlign: 'middle',
                          color: '#71717A',
                        }}
                      >
                        Actions
                      </th>
                    </tr>
                  </thead>
                  <tbody style={{fontFamily: 'Geist', fontSize: '14px'}}>
                    {[
                      {
                        name: 'Main Company Wallet',
                        address: '0x1a2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t',
                        network: 'Ethereum',
                        status: 'Default',
                      },
                      {
                        name: 'Payroll Wallet',
                        address: '0x9s8r7q6p5o4n3m2l1k0j9i8h7g6f5e4d3c2b1a0',
                        network: 'Polygon',
                        status: 'Connected',
                      },
                      {
                        name: 'Vendor Payments',
                        address: '0x2b3c4d5e6f7g8h9i0j1k2l3m4n5o6p7q8r9s0t1',
                        network: 'Optimism',
                        status: 'Connected',
                      },
                    ].map((wallet) => (
                      <tr key={wallet.name} className="border-b last:border-b-0">
                        <td
                          className="px-4 py-3 whitespace-nowrap"
                          style={{
                            fontFamily: 'Geist',
                            fontWeight: 500,
                            fontSize: '14px',
                            lineHeight: '20px',
                            letterSpacing: '0%',
                            verticalAlign: 'middle',
                            color: '#09090B',
                          }}
                        >
                          {wallet.name}
                        </td>
                        <td className="px-4 py-3 flex items-center gap-2 whitespace-nowrap">
                          <span
                            className="truncate max-w-[160px]"
                            title={wallet.address}
                            style={{
                              fontFamily: 'Inter',
                              fontWeight: 400,
                              fontSize: '11.81px',
                              lineHeight: '16px',
                              letterSpacing: '0%',
                              verticalAlign: 'middle',
                              color: '#09090B',
                            }}
                          >
                            {wallet.address}
                          </span>
                          <button className="p-1 hover:bg-gray-100 rounded" title="Copy address">
                            <Copy size={16} className="text-gray-400" />
                          </button>
                        </td>
                        <td className="px-4 py-3 whitespace-nowrap">{wallet.network}</td>
                        <td className="px-4 py-3 whitespace-nowrap">
                          {wallet.status === 'Default' ? (
                            <span className="bg-green-50 text-green-600 px-3 py-1 rounded-full text-xs font-medium">Default</span>
                          ) : (
                            <span className="bg-gray-100 text-gray-600 px-3 py-1 rounded-full text-xs font-medium">Connected</span>
                          )}
                        </td>
                        <td className="px-4 py-3 flex gap-2 whitespace-nowrap">
                          <button className="border border-gray-200 rounded px-3 py-1 text-xs font-medium hover:bg-gray-50">Edit</button>
                          {wallet.status !== 'Default' && (
                            <button className="border border-gray-200 rounded px-3 py-1 text-xs font-medium hover:bg-gray-50">Set Default</button>
                          )}
                          <button className="p-1 hover:bg-gray-100 rounded" title="Delete wallet">
                            <Trash2 size={16} className="text-red-400" />
                          </button>
                        </td>
                      </tr>
                    ))}
                  </tbody>
                </table>
              </div>
              <div>
                <h3
                  style={{
                    fontFamily: 'Geist',
                    fontWeight: 500,
                    fontSize: '18px',
                    lineHeight: '28px',
                    letterSpacing: '0%',
                    verticalAlign: 'middle',
                    color: '#09090B',
                  }}
                  className="mb-4"
                >Add New Wallet</h3>
                <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1" style={{fontFamily: 'Geist'}}>Wallet Name</label>
                    <input
                      type="text"
                      placeholder="e.g., Marketing Expenses"
                      className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300"
                      style={{fontFamily: 'Geist', fontWeight: 400, fontSize: '14px', lineHeight: '100%'}}
                    />
                  </div>
                  <div>
                    <label className="block text-sm font-medium text-gray-700 mb-1" style={{fontFamily: 'Geist'}}>Network</label>
                    <input
                      type="text"
                      placeholder="e.g., Ethereum, Polygon"
                      className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300"
                      style={{fontFamily: 'Geist', fontWeight: 400, fontSize: '14px', lineHeight: '100%'}}
                    />
                  </div>
                </div>
                <div className="mb-6">
                  <label className="block text-sm font-medium text-gray-700 mb-1" style={{fontFamily: 'Geist'}}>Wallet Address</label>
                  <input
                    type="text"
                    placeholder="0x..."
                    className="w-full border border-gray-200 rounded-md px-3 py-2 text-sm focus:outline-none focus:ring-2 focus:ring-gray-300"
                    style={{fontFamily: 'Geist', fontWeight: 400, fontSize: '14px', lineHeight: '100%'}}
                  />
                </div>
                <div className="flex justify-end gap-4">
                  <button className="px-4 py-2 rounded-md text-sm font-medium text-gray-700 hover:bg-gray-100 transition-colors">Cancel</button>
                  <button
                    style={{
                      fontFamily: 'Geist',
                      fontWeight: 500,
                      fontSize: '14px',
                      lineHeight: '20px',
                      letterSpacing: '0%',
                      textAlign: 'center',
                      verticalAlign: 'middle',
                      color: '#FAFAFA',
                      backgroundColor: '#18181B',
                    }}
                    className="px-4 py-2 rounded-md text-sm font-medium hover:opacity-90 transition-opacity"
                  >
                    Save Wallet
                  </button>
                </div>
              </div>
            </>
          ) : (
            <div className="bg-white rounded-lg shadow p-6 flex items-center justify-center min-h-[200px] max-w-4xl mx-auto">
              <span className="text-gray-400 text-lg font-medium">Coming soon...</span>
            </div>
          )}
        </div>
      </div>
    </main>
  );
}
