"use client";

import React from "react";
import { Switch } from "@/app/components/ui/Switch";
import { ChevronDown } from "lucide-react";
import {
  Tabs,
  TabsList,
  TabsTrigger,
  TabsContent,
} from "@/app/components/ui/Tabs";

export default function Settings() {
  // State for notification preferences
  const [emailPrefs, setEmailPrefs] = React.useState({
    batchCreated: false,
    batchProcessed: false,
    batchFailed: false,
    weeklySummary: false,
  });

  const [pushPrefs, setPushPrefs] = React.useState({
    statusChanges: false,
    criticalAlerts: false,
  });

  const [frequency, setFrequency] = React.useState("immediate");
  const [quietHours, setQuietHours] = React.useState("none");

  // Toggle handlers
  const handleEmailPrefChange = (key: keyof typeof emailPrefs) => {
    setEmailPrefs((prev) => ({ ...prev, [key]: !prev[key] }));
  };

  const handlePushPrefChange = (key: keyof typeof pushPrefs) => {
    setPushPrefs((prev) => ({ ...prev, [key]: !prev[key] }));
  };
  return (
    <main className="min-h-screen bg-gray-50">
      <div className="max-w-[1171px] mx-auto py-8 px-4">
        <h1 className="text-3xl font-semibold text-gray-900">Settings</h1>
        <p className="mt-1 mb-6 text-base text-gray-500">Manage your account settings and preferences</p>

        <Tabs defaultValue="notifications">
          <TabsList className="w-[456.69px]">
            <TabsTrigger value="profile">Profile</TabsTrigger>
            <TabsTrigger value="notifications">Notifications</TabsTrigger>
            <TabsTrigger value="wallets">Wallets</TabsTrigger>
            <TabsTrigger value="api">API Keys</TabsTrigger>
          </TabsList>

          <TabsContent value="profile" />

          <TabsContent value="notifications">
            <div className="w-[456.69px] min-h-[764px] ml-[32px] border border-gray-200 rounded-[8px] p-6 bg-white">
              <h1 className="text-2xl font-semibold text-gray-900">
                Notification Preferences
              </h1>
              <p className="mt-1 text-sm text-gray-500">
                Manage how and when you receive notifications
              </p>

              {/* Email Notifications */}
              <section className="mt-8">
                <h2 className="text-lg font-medium text-gray-900 mb-4">
                  Email Notifications
                </h2>
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-medium text-gray-900">
                        Batch Created
                      </h3>
                      <p className="text-sm text-gray-500">
                        Receive an email when a new batch payment is created
                      </p>
                    </div>
                    <Switch
                      checked={emailPrefs.batchCreated}
                      onCheckedChange={() =>
                        handleEmailPrefChange("batchCreated")
                      }
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-medium text-gray-900">
                        Batch Processed
                      </h3>
                      <p className="text-sm text-gray-500">
                        Receive an email when a batch is processed
                      </p>
                    </div>
                    <Switch
                      checked={emailPrefs.batchProcessed}
                      onCheckedChange={() =>
                        handleEmailPrefChange("batchProcessed")
                      }
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-medium text-gray-900">
                        Batch Failed
                      </h3>
                      <p className="text-sm text-gray-500">
                        Receive an email when a batch payment fails
                      </p>
                    </div>
                    <Switch
                      checked={emailPrefs.batchFailed}
                      onCheckedChange={() =>
                        handleEmailPrefChange("batchFailed")
                      }
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-medium text-gray-900">
                        Weekly Summary
                      </h3>
                      <p className="text-sm text-gray-500">
                        Receive a weekly summary of all batch payment activity
                      </p>
                    </div>
                    <Switch
                      checked={emailPrefs.weeklySummary}
                      onCheckedChange={() =>
                        handleEmailPrefChange("weeklySummary")
                      }
                    />
                  </div>
                </div>
              </section>

              {/* Push Notifications */}
              <section className="mt-8">
                <h2 className="text-lg font-medium text-gray-900 mb-4">
                  Push Notifications
                </h2>
                <div className="space-y-4">
                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-medium text-gray-900">
                        Batch Status Changes
                      </h3>
                      <p className="text-sm text-gray-500">
                        Receive push notifications when batch state updates
                      </p>
                    </div>
                    <Switch
                      checked={pushPrefs.statusChanges}
                      onCheckedChange={() =>
                        handlePushPrefChange("statusChanges")
                      }
                    />
                  </div>

                  <div className="flex items-center justify-between">
                    <div>
                      <h3 className="text-sm font-medium text-gray-900">
                        Critical Alerts
                      </h3>
                      <p className="text-sm text-gray-500">
                        Receive push notifications for high-priority system
                        alerts
                      </p>
                    </div>
                    <Switch
                      checked={pushPrefs.criticalAlerts}
                      onCheckedChange={() =>
                        handlePushPrefChange("criticalAlerts")
                      }
                    />
                  </div>
                </div>
              </section>

              {/* Notification Delivery */}
              <section className="mt-8">
                <h2 className="text-lg font-medium text-gray-900 mb-4">
                  Notification Delivery
                </h2>
                <div className="grid grid-cols-2 gap-6">
                  <div>
                    <label className="block text-sm font-medium text-gray-700">
                      Email Frequency
                    </label>
                    <div className="relative mt-1">
                      <select
                        value={frequency}
                        onChange={(e) => setFrequency(e.target.value)}
                        className="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary focus:border-primary text-sm"
                      >
                        <option value="immediate">Immediate</option>
                        <option value="hourly">Hourly</option>
                        <option value="daily">Daily</option>
                        <option value="weekly">Weekly</option>
                      </select>
                      <ChevronDown className="absolute right-3 top-2.5 h-4 w-4 text-gray-400" />
                    </div>
                  </div>

                  <div>
                    <label className="block text-sm font-medium text-gray-700">
                      Quiet Hours
                    </label>
                    <div className="relative mt-1">
                      <select
                        value={quietHours}
                        onChange={(e) => setQuietHours(e.target.value)}
                        className="appearance-none block w-full px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-primary focus:border-primary text-sm"
                      >
                        <option value="none">No quiet hours</option>
                        <option value="night">Night only (10PM - 6AM)</option>
                        <option value="custom">Custom schedule</option>
                      </select>
                      <ChevronDown className="absolute right-3 top-2.5 h-4 w-4 text-gray-400" />
                    </div>
                  </div>
                </div>
              </section>

              {/* Save Button */}
              <div className="mt-8 pt-8 border-t border-gray-200">
                <button
                  type="button"
                  className="bg-primary text-white px-4 py-2 rounded-md text-sm font-medium hover:bg-opacity-90 focus:outline-none focus:ring-2 focus:ring-offset-2 focus:ring-primary"
                >
                  Save Preferences
                </button>
              </div>
            </div>
          </TabsContent>

          <TabsContent value="wallets" />
          <TabsContent value="api" />
        </Tabs>
      </div>
    </main>
  );
}
