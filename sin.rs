pub trait ToGoogleScope {
    fn to_google_scope(&self) -> &'static str;
}



pub enum AIPlatformTrainingAndPredictionAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
}



pub enum AccessApprovalAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum AccessContextManagerAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum AdExchangeBuyerAPIIIv2beta1 {
    
    /// Documentation: Manage your Ad Exchange buyer account configuration, Scope: https://www.googleapis.com/auth/adexchange.buyer
    AuthAdexchangeBuyer,
    
}



pub enum AdMobAPIv1 {
    
    /// Documentation: Manage your Ad Exchange buyer account configuration, Scope: https://www.googleapis.com/auth/adexchange.buyer
    AuthAdexchangeBuyer,
    
    /// Documentation: See your AdMob data, Scope: https://www.googleapis.com/auth/admob.report
    AuthAdmobReport,
    
}



pub enum AdSenseHostAPIv4Point1 {
    
    /// Documentation: View and manage your AdSense host data and associated accounts, Scope: https://www.googleapis.com/auth/adsensehost
    AuthAdsensehost,
    
}



pub enum AdminSDKAPIv1 {
    
    /// Documentation: View audit reports for your G Suite domain, Scope: https://www.googleapis.com/auth/admin.reports.audit.readonly
    AuthAdminReportsAuditReadOnly,
    
    /// Documentation: View usage reports for your G Suite domain, Scope: https://www.googleapis.com/auth/admin.reports.usage.readonly
    AuthAdminReportsUsageReadOnly,
    
    /// Documentation: View and manage data transfers between users in your organization, Scope: https://www.googleapis.com/auth/admin.datatransfer
    AuthAdminDatatransfer,
    
    /// Documentation: View data transfers between users in your organization, Scope: https://www.googleapis.com/auth/admin.datatransfer.readonly
    AuthAdminDatatransferReadOnly,
    
    /// Documentation: See, add, edit, and permanently delete the printers that your organization can use with Chrome, Scope: https://www.googleapis.com/auth/admin.chrome.printers
    AuthAdminChromePrinters,
    
    /// Documentation: See the printers that your organization can use with Chrome, Scope: https://www.googleapis.com/auth/admin.chrome.printers.readonly
    AuthAdminChromePrintersReadOnly,
    
    /// Documentation: View and manage customer related information, Scope: https://www.googleapis.com/auth/admin.directory.customer
    AuthAdminDirectoryCustomer,
    
    /// Documentation: View customer related information, Scope: https://www.googleapis.com/auth/admin.directory.customer.readonly
    AuthAdminDirectoryCustomerReadOnly,
    
    /// Documentation: View and manage your Chrome OS devices' metadata, Scope: https://www.googleapis.com/auth/admin.directory.device.chromeos
    AuthAdminDirectoryDeviceChromeos,
    
    /// Documentation: View your Chrome OS devices' metadata, Scope: https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly
    AuthAdminDirectoryDeviceChromeosReadOnly,
    
    /// Documentation: View and manage your mobile devices' metadata, Scope: https://www.googleapis.com/auth/admin.directory.device.mobile
    AuthAdminDirectoryDeviceMobile,
    
    /// Documentation: Manage your mobile devices by performing administrative tasks, Scope: https://www.googleapis.com/auth/admin.directory.device.mobile.action
    AuthAdminDirectoryDeviceMobileAction,
    
    /// Documentation: View your mobile devices' metadata, Scope: https://www.googleapis.com/auth/admin.directory.device.mobile.readonly
    AuthAdminDirectoryDeviceMobileReadOnly,
    
    /// Documentation: View and manage the provisioning of domains for your customers, Scope: https://www.googleapis.com/auth/admin.directory.domain
    AuthAdminDirectoryDomain,
    
    /// Documentation: View domains related to your customers, Scope: https://www.googleapis.com/auth/admin.directory.domain.readonly
    AuthAdminDirectoryDomainReadOnly,
    
    /// Documentation: View and manage the provisioning of groups on your domain, Scope: https://www.googleapis.com/auth/admin.directory.group
    AuthAdminDirectoryGroup,
    
    /// Documentation: View and manage group subscriptions on your domain, Scope: https://www.googleapis.com/auth/admin.directory.group.member
    AuthAdminDirectoryGroupMember,
    
    /// Documentation: View group subscriptions on your domain, Scope: https://www.googleapis.com/auth/admin.directory.group.member.readonly
    AuthAdminDirectoryGroupMemberReadOnly,
    
    /// Documentation: View groups on your domain, Scope: https://www.googleapis.com/auth/admin.directory.group.readonly
    AuthAdminDirectoryGroupReadOnly,
    
    /// Documentation: View and manage organization units on your domain, Scope: https://www.googleapis.com/auth/admin.directory.orgunit
    AuthAdminDirectoryOrgunit,
    
    /// Documentation: View organization units on your domain, Scope: https://www.googleapis.com/auth/admin.directory.orgunit.readonly
    AuthAdminDirectoryOrgunitReadOnly,
    
    /// Documentation: View and manage the provisioning of calendar resources on your domain, Scope: https://www.googleapis.com/auth/admin.directory.resource.calendar
    AuthAdminDirectoryResourceCalendar,
    
    /// Documentation: View calendar resources on your domain, Scope: https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly
    AuthAdminDirectoryResourceCalendarReadOnly,
    
    /// Documentation: Manage delegated admin roles for your domain, Scope: https://www.googleapis.com/auth/admin.directory.rolemanagement
    AuthAdminDirectoryRolemanagement,
    
    /// Documentation: View delegated admin roles for your domain, Scope: https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly
    AuthAdminDirectoryRolemanagementReadOnly,
    
    /// Documentation: View and manage the provisioning of users on your domain, Scope: https://www.googleapis.com/auth/admin.directory.user
    AuthAdminDirectoryUser,
    
    /// Documentation: View and manage user aliases on your domain, Scope: https://www.googleapis.com/auth/admin.directory.user.alias
    AuthAdminDirectoryUserAlias,
    
    /// Documentation: View user aliases on your domain, Scope: https://www.googleapis.com/auth/admin.directory.user.alias.readonly
    AuthAdminDirectoryUserAliasReadOnly,
    
    /// Documentation: See info about users on your domain, Scope: https://www.googleapis.com/auth/admin.directory.user.readonly
    AuthAdminDirectoryUserReadOnly,
    
    /// Documentation: Manage data access permissions for users on your domain, Scope: https://www.googleapis.com/auth/admin.directory.user.security
    AuthAdminDirectoryUserSecurity,
    
    /// Documentation: View and manage the provisioning of user schemas on your domain, Scope: https://www.googleapis.com/auth/admin.directory.userschema
    AuthAdminDirectoryUserschema,
    
    /// Documentation: View user schemas on your domain, Scope: https://www.googleapis.com/auth/admin.directory.userschema.readonly
    AuthAdminDirectoryUserschemaReadOnly,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum AnalyticsReportingAPIv4 {
    
    /// Documentation: View and manage your Google Analytics data, Scope: https://www.googleapis.com/auth/analytics
    AuthAnalytics,
    
    /// Documentation: See and download your Google Analytics data, Scope: https://www.googleapis.com/auth/analytics.readonly
    AuthAnalyticsReadOnly,
    
}



pub enum AndroidManagementAPIv1 {
    
    /// Documentation: Manage Android devices and apps for your customers, Scope: https://www.googleapis.com/auth/androidmanagement
    AuthAndroidmanagement,
    
}



pub enum ApigeeAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum AppEngineAdminAPIv1 {
    
    /// Documentation: View and manage your applications deployed on Google App Engine, Scope: https://www.googleapis.com/auth/appengine.admin
    AuthAppengineAdmin,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
}



pub enum AppsScriptAPIv1 {
    
    /// Documentation: Read, compose, send, and permanently delete all your email from Gmail, Scope: https://mail.google.com/
    /,
    
    /// Documentation: See, edit, share, and permanently delete all the calendars you can access using Google Calendar, Scope: https://www.google.com/calendar/feeds
    CalendarFeeds,
    
    /// Documentation: See, edit, download, and permanently delete your contacts, Scope: https://www.google.com/m8/feeds
    M8Feeds,
    
    /// Documentation: View and manage the provisioning of groups on your domain, Scope: https://www.googleapis.com/auth/admin.directory.group
    AuthAdminDirectoryGroup,
    
    /// Documentation: View and manage the provisioning of users on your domain, Scope: https://www.googleapis.com/auth/admin.directory.user
    AuthAdminDirectoryUser,
    
    /// Documentation: See, edit, create, and delete all your Google Docs documents, Scope: https://www.googleapis.com/auth/documents
    AuthDocuments,
    
    /// Documentation: See, edit, create, and delete all of your Google Drive files, Scope: https://www.googleapis.com/auth/drive
    AuthDrive,
    
    /// Documentation: View and manage your forms in Google Drive, Scope: https://www.googleapis.com/auth/forms
    AuthForms,
    
    /// Documentation: View and manage forms that this application has been installed in, Scope: https://www.googleapis.com/auth/forms.currentonly
    AuthFormsCurrentOnly,
    
    /// Documentation: View and manage your Google Groups, Scope: https://www.googleapis.com/auth/groups
    AuthGroups,
    
    /// Documentation: Create and update Google Apps Script deployments, Scope: https://www.googleapis.com/auth/script.deployments
    AuthScriptDeployments,
    
    /// Documentation: View Google Apps Script deployments, Scope: https://www.googleapis.com/auth/script.deployments.readonly
    AuthScriptDeploymentsReadOnly,
    
    /// Documentation: View Google Apps Script project's metrics, Scope: https://www.googleapis.com/auth/script.metrics
    AuthScriptMetrics,
    
    /// Documentation: View Google Apps Script processes, Scope: https://www.googleapis.com/auth/script.processes
    AuthScriptProcesses,
    
    /// Documentation: Create and update Google Apps Script projects, Scope: https://www.googleapis.com/auth/script.projects
    AuthScriptProjects,
    
    /// Documentation: View Google Apps Script projects, Scope: https://www.googleapis.com/auth/script.projects.readonly
    AuthScriptProjectsReadOnly,
    
    /// Documentation: See, edit, create, and delete all your Google Sheets spreadsheets, Scope: https://www.googleapis.com/auth/spreadsheets
    AuthSpreadsheets,
    
    /// Documentation: See your primary Google Account email address, Scope: https://www.googleapis.com/auth/userinfo.email
    AuthUserinfoEmail,
    
}



pub enum BigQueryAPIv2 {
    
    /// Documentation: View and manage your data in Google BigQuery and see the email address for your Google Account, Scope: https://www.googleapis.com/auth/bigquery
    AuthBigquery,
    
    /// Documentation: Insert data into Google BigQuery, Scope: https://www.googleapis.com/auth/bigquery.insertdata
    AuthBigqueryInsertdata,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: Manage your data and permissions in Cloud Storage and see the email address for your Google Account, Scope: https://www.googleapis.com/auth/devstorage.full_control
    AuthDevstorageFull_Control,
    
    /// Documentation: View your data in Google Cloud Storage, Scope: https://www.googleapis.com/auth/devstorage.read_only
    AuthDevstorageRead_Only,
    
    /// Documentation: Manage your data in Cloud Storage and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/devstorage.read_write
    AuthDevstorageRead_Write,
    
}



pub enum BigQueryConnectionAPIv1beta1 {
    
    /// Documentation: View and manage your data in Google BigQuery and see the email address for your Google Account, Scope: https://www.googleapis.com/auth/bigquery
    AuthBigquery,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum BigQueryDataTransferAPIv1 {
    
    /// Documentation: View and manage your data in Google BigQuery and see the email address for your Google Account, Scope: https://www.googleapis.com/auth/bigquery
    AuthBigquery,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
}



pub enum BigQueryReservationAPIv1 {
    
    /// Documentation: View and manage your data in Google BigQuery and see the email address for your Google Account, Scope: https://www.googleapis.com/auth/bigquery
    AuthBigquery,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum BinaryAuthorizationAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum BloggerAPIv3 {
    
    /// Documentation: Manage your Blogger account, Scope: https://www.googleapis.com/auth/blogger
    AuthBlogger,
    
    /// Documentation: View your Blogger account, Scope: https://www.googleapis.com/auth/blogger.readonly
    AuthBloggerReadOnly,
    
}



pub enum BooksAPIv1 {
    
    /// Documentation: Manage your books, Scope: https://www.googleapis.com/auth/books
    AuthBooks,
    
}



pub enum CalendarAPIv3 {
    
    /// Documentation: See, edit, share, and permanently delete all the calendars you can access using Google Calendar, Scope: https://www.googleapis.com/auth/calendar
    AuthCalendar,
    
    /// Documentation: View and edit events on all your calendars, Scope: https://www.googleapis.com/auth/calendar.events
    AuthCalendarEvents,
    
    /// Documentation: View events on all your calendars, Scope: https://www.googleapis.com/auth/calendar.events.readonly
    AuthCalendarEventsReadOnly,
    
    /// Documentation: See and download any calendar you can access using your Google Calendar, Scope: https://www.googleapis.com/auth/calendar.readonly
    AuthCalendarReadOnly,
    
    /// Documentation: View your Calendar settings, Scope: https://www.googleapis.com/auth/calendar.settings.readonly
    AuthCalendarSettingsReadOnly,
    
}



pub enum CampaignManager360APIv4 {
    
    /// Documentation: Manage DoubleClick Digital Marketing conversions, Scope: https://www.googleapis.com/auth/ddmconversions
    AuthDdmconversions,
    
    /// Documentation: View and manage DoubleClick for Advertisers reports, Scope: https://www.googleapis.com/auth/dfareporting
    AuthDfareporting,
    
    /// Documentation: View and manage your DoubleClick Campaign Manager's (DCM) display ad campaigns, Scope: https://www.googleapis.com/auth/dfatrafficking
    AuthDfatrafficking,
    
}



pub enum CloudAssetAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudBigtableAdminAPIv2 {
    
    /// Documentation: Administer your Cloud Bigtable tables and clusters, Scope: https://www.googleapis.com/auth/bigtable.admin
    AuthBigtableAdmin,
    
    /// Documentation: Administer your Cloud Bigtable clusters, Scope: https://www.googleapis.com/auth/bigtable.admin.cluster
    AuthBigtableAdminCluster,
    
    /// Documentation: Administer your Cloud Bigtable clusters, Scope: https://www.googleapis.com/auth/bigtable.admin.instance
    AuthBigtableAdminInstance,
    
    /// Documentation: Administer your Cloud Bigtable tables, Scope: https://www.googleapis.com/auth/bigtable.admin.table
    AuthBigtableAdminTable,
    
    /// Documentation: Administer your Cloud Bigtable tables and clusters, Scope: https://www.googleapis.com/auth/cloud-bigtable.admin
    AuthCloudBigtableAdmin,
    
    /// Documentation: Administer your Cloud Bigtable clusters, Scope: https://www.googleapis.com/auth/cloud-bigtable.admin.cluster
    AuthCloudBigtableAdminCluster,
    
    /// Documentation: Administer your Cloud Bigtable tables, Scope: https://www.googleapis.com/auth/cloud-bigtable.admin.table
    AuthCloudBigtableAdminTable,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
}



pub enum CloudBillingAPIv1 {
    
    /// Documentation: View and manage your Google Cloud Platform billing accounts, Scope: https://www.googleapis.com/auth/cloud-billing
    AuthCloudBilling,
    
    /// Documentation: View your Google Cloud Platform billing accounts, Scope: https://www.googleapis.com/auth/cloud-billing.readonly
    AuthCloudBillingReadOnly,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudBuildAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudComposerAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudDNSAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: View your DNS records hosted by Google Cloud DNS, Scope: https://www.googleapis.com/auth/ndev.clouddns.readonly
    AuthNdevClouddnsReadOnly,
    
    /// Documentation: View and manage your DNS records hosted by Google Cloud DNS, Scope: https://www.googleapis.com/auth/ndev.clouddns.readwrite
    AuthNdevClouddnsReadwrite,
    
}



pub enum CloudDataLossPrevention(DLP)APIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudDataprocAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudDatastoreAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage your Google Cloud Datastore data, Scope: https://www.googleapis.com/auth/datastore
    AuthDatastore,
    
}



pub enum CloudDebuggerAPIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Use Stackdriver Debugger, Scope: https://www.googleapis.com/auth/cloud_debugger
    AuthCloud_Debugger,
    
}



pub enum CloudDeploymentManagerV2APIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: View and manage your Google Cloud Platform management resources and deployment status information, Scope: https://www.googleapis.com/auth/ndev.cloudman
    AuthNdevCloudman,
    
    /// Documentation: View your Google Cloud Platform management resources and deployment status information, Scope: https://www.googleapis.com/auth/ndev.cloudman.readonly
    AuthNdevCloudmanReadOnly,
    
}



pub enum CloudFilestoreAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudFirestoreAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage your Google Cloud Datastore data, Scope: https://www.googleapis.com/auth/datastore
    AuthDatastore,
    
}



pub enum CloudHealthcareAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudIdentity-AwareProxyAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudIdentityAPIv1 {
    
    /// Documentation: See your device details, Scope: https://www.googleapis.com/auth/cloud-identity.devices.lookup
    AuthCloudIdentityDevicesLookup,
    
    /// Documentation: See, change, create, and delete any of the Cloud Identity Groups that you can access, including the members of each group, Scope: https://www.googleapis.com/auth/cloud-identity.groups
    AuthCloudIdentityGroups,
    
    /// Documentation: See any Cloud Identity Groups that you can access, including group members and their emails, Scope: https://www.googleapis.com/auth/cloud-identity.groups.readonly
    AuthCloudIdentityGroupsReadOnly,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudKeyManagementService(KMS)APIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage your keys and secrets stored in Cloud Key Management Service, Scope: https://www.googleapis.com/auth/cloudkms
    AuthCloudkms,
    
}



pub enum CloudLifeSciencesAPIv2beta {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudLoggingAPIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: Administrate log data for your projects, Scope: https://www.googleapis.com/auth/logging.admin
    AuthLoggingAdmin,
    
    /// Documentation: View log data for your projects, Scope: https://www.googleapis.com/auth/logging.read
    AuthLoggingRead,
    
    /// Documentation: Submit log data for your projects, Scope: https://www.googleapis.com/auth/logging.write
    AuthLoggingWrite,
    
}



pub enum CloudMonitoringAPIv3 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and write monitoring data for all of your Google and third-party Cloud and API projects, Scope: https://www.googleapis.com/auth/monitoring
    AuthMonitoring,
    
    /// Documentation: View monitoring data for all of your Google Cloud and third-party projects, Scope: https://www.googleapis.com/auth/monitoring.read
    AuthMonitoringRead,
    
    /// Documentation: Publish metric data to your Google Cloud projects, Scope: https://www.googleapis.com/auth/monitoring.write
    AuthMonitoringWrite,
    
}



pub enum CloudNaturalLanguageAPIv1 {
    
    /// Documentation: Apply machine learning models to reveal the structure and meaning of text, Scope: https://www.googleapis.com/auth/cloud-language
    AuthCloudLanguage,
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudOSLoginAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: View and manage your Google Compute Engine resources, Scope: https://www.googleapis.com/auth/compute
    AuthCompute,
    
    /// Documentation: View your Google Compute Engine resources, Scope: https://www.googleapis.com/auth/compute.readonly
    AuthComputeReadOnly,
    
}



pub enum CloudProfilerAPIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and write monitoring data for all of your Google and third-party Cloud and API projects, Scope: https://www.googleapis.com/auth/monitoring
    AuthMonitoring,
    
    /// Documentation: Publish metric data to your Google Cloud projects, Scope: https://www.googleapis.com/auth/monitoring.write
    AuthMonitoringWrite,
    
}



pub enum CloudPub/SubAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage Pub/Sub topics and subscriptions, Scope: https://www.googleapis.com/auth/pubsub
    AuthPubsub,
    
}



pub enum CloudResourceManagerAPIv3 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
}



pub enum CloudRuntimeConfigurationAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Manage your Google Cloud Platform services' runtime configuration, Scope: https://www.googleapis.com/auth/cloudruntimeconfig
    AuthCloudruntimeconfig,
    
}



pub enum CloudSQLAdminAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Manage your Google SQL Service instances, Scope: https://www.googleapis.com/auth/sqlservice.admin
    AuthSqlserviceAdmin,
    
}



pub enum CloudSchedulerAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudSearchAPIv1 {
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search
    AuthCloud_Search,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.debug
    AuthCloud_SearchDebug,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.indexing
    AuthCloud_SearchIndexing,
    
    /// Documentation: Search your organization's data in the Cloud Search index, Scope: https://www.googleapis.com/auth/cloud_search.query
    AuthCloud_SearchQuery,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.settings
    AuthCloud_SearchSettings,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.settings.indexing
    AuthCloud_SearchSettingsIndexing,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.settings.query
    AuthCloud_SearchSettingsQuery,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.stats
    AuthCloud_SearchStats,
    
    /// Documentation: Index and serve your organization's data with Cloud Search, Scope: https://www.googleapis.com/auth/cloud_search.stats.indexing
    AuthCloud_SearchStatsIndexing,
    
}



pub enum CloudShellAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudSourceRepositoriesAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Manage your source code repositories, Scope: https://www.googleapis.com/auth/source.full_control
    AuthSourceFull_Control,
    
    /// Documentation: View the contents of your source code repositories, Scope: https://www.googleapis.com/auth/source.read_only
    AuthSourceRead_Only,
    
    /// Documentation: Manage the contents of your source code repositories, Scope: https://www.googleapis.com/auth/source.read_write
    AuthSourceRead_Write,
    
}



pub enum CloudSpannerAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Administer your Spanner databases, Scope: https://www.googleapis.com/auth/spanner.admin
    AuthSpannerAdmin,
    
    /// Documentation: View and manage the contents of your Spanner databases, Scope: https://www.googleapis.com/auth/spanner.data
    AuthSpannerData,
    
}



pub enum CloudSpeech-to-TextAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudStorageJSONAPIv1 {
    
    /// Documentation: View and manage your data across Google Cloud Platform services, Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud Platform services, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: Manage your data and permissions in Google Cloud Storage, Scope: https://www.googleapis.com/auth/devstorage.full_control
    AuthDevstorageFull_Control,
    
    /// Documentation: View your data in Google Cloud Storage, Scope: https://www.googleapis.com/auth/devstorage.read_only
    AuthDevstorageRead_Only,
    
    /// Documentation: Manage your data in Google Cloud Storage, Scope: https://www.googleapis.com/auth/devstorage.read_write
    AuthDevstorageRead_Write,
    
}



pub enum CloudTasksAPIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudTestingAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
}



pub enum CloudText-to-SpeechAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudToolResultsAPIv1beta3 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudTraceAPIv2 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Write Trace data for a project or application, Scope: https://www.googleapis.com/auth/trace.append
    AuthTraceAppend,
    
}



pub enum CloudTranslationAPIv3 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Translate text from one language to another using Google Translate, Scope: https://www.googleapis.com/auth/cloud-translation
    AuthCloudTranslation,
    
}



pub enum CloudVideoIntelligenceAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum CloudVisionAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Apply machine learning models to understand and label images, Scope: https://www.googleapis.com/auth/cloud-vision
    AuthCloudVision,
    
}



pub enum ComputeEngineAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage your Google Compute Engine resources, Scope: https://www.googleapis.com/auth/compute
    AuthCompute,
    
    /// Documentation: View your Google Compute Engine resources, Scope: https://www.googleapis.com/auth/compute.readonly
    AuthComputeReadOnly,
    
    /// Documentation: Manage your data and permissions in Cloud Storage and see the email address for your Google Account, Scope: https://www.googleapis.com/auth/devstorage.full_control
    AuthDevstorageFull_Control,
    
    /// Documentation: View your data in Google Cloud Storage, Scope: https://www.googleapis.com/auth/devstorage.read_only
    AuthDevstorageRead_Only,
    
    /// Documentation: Manage your data in Cloud Storage and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/devstorage.read_write
    AuthDevstorageRead_Write,
    
}



pub enum ContentAPIforShoppingv2Point1 {
    
    /// Documentation: Manage your product listings and accounts for Google Shopping, Scope: https://www.googleapis.com/auth/content
    AuthContent,
    
}



pub enum DataPortabilityAPIv1 {
    
    /// Documentation: Move a copy of messages between you and the businesses you have conversations with across Google services., Scope: https://www.googleapis.com/auth/dataportability.businessmessaging.conversations
    AuthDataportabilityBusinessmessagingConversations,
    
    /// Documentation: Move a copy of the information you entered into online forms in Chrome., Scope: https://www.googleapis.com/auth/dataportability.chrome.autofill
    AuthDataportabilityChromeAutofill,
    
    /// Documentation: Move a copy of pages you bookmarked in Chrome., Scope: https://www.googleapis.com/auth/dataportability.chrome.bookmarks
    AuthDataportabilityChromeBookmarks,
    
    /// Documentation: Move a copy of words you added to Chrome's dictionary., Scope: https://www.googleapis.com/auth/dataportability.chrome.dictionary
    AuthDataportabilityChromeDictionary,
    
    /// Documentation: Move a copy of extensions you installed from the Chrome Web Store., Scope: https://www.googleapis.com/auth/dataportability.chrome.extensions
    AuthDataportabilityChromeExtensions,
    
    /// Documentation: Move a copy of sites you visited in Chrome., Scope: https://www.googleapis.com/auth/dataportability.chrome.history
    AuthDataportabilityChromeHistory,
    
    /// Documentation: Move a copy of pages you added to your reading list in Chrome., Scope: https://www.googleapis.com/auth/dataportability.chrome.reading_list
    AuthDataportabilityChromeReading_List,
    
    /// Documentation: Move a copy of your settings in Chrome., Scope: https://www.googleapis.com/auth/dataportability.chrome.settings
    AuthDataportabilityChromeSettings,
    
    /// Documentation: Move a copy of your pinned trips on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.commute_routes
    AuthDataportabilityMapsCommute_Routes,
    
    /// Documentation: Move a copy of your commute settings on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.commute_settings
    AuthDataportabilityMapsCommute_Settings,
    
    /// Documentation: Move a copy of your electric vehicle profile on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.ev_profile
    AuthDataportabilityMapsEv_Profile,
    
    /// Documentation: Move a copy of your updates to places on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.offering_contributions
    AuthDataportabilityMapsOffering_Contributions,
    
    /// Documentation: Move a copy of the photos and videos you posted on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.photos_videos
    AuthDataportabilityMapsPhotos_Videos,
    
    /// Documentation: Move a copy of your reviews and posts on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.reviews
    AuthDataportabilityMapsReviews,
    
    /// Documentation: Move a copy of your Starred places list on Maps., Scope: https://www.googleapis.com/auth/dataportability.maps.starred_places
    AuthDataportabilityMapsStarred_Places,
    
    /// Documentation: Move a copy of your Maps activity., Scope: https://www.googleapis.com/auth/dataportability.myactivity.maps
    AuthDataportabilityMyactivityMaps,
    
    /// Documentation: Move a copy of your Google Search activity., Scope: https://www.googleapis.com/auth/dataportability.myactivity.search
    AuthDataportabilityMyactivitySearch,
    
    /// Documentation: Move a copy of your Shopping activity., Scope: https://www.googleapis.com/auth/dataportability.myactivity.shopping
    AuthDataportabilityMyactivityShopping,
    
    /// Documentation: Move a copy of your YouTube activity., Scope: https://www.googleapis.com/auth/dataportability.myactivity.youtube
    AuthDataportabilityMyactivityYoutube,
    
    /// Documentation: Move a copy of your saved links, images, places, and collections from your use of Google services., Scope: https://www.googleapis.com/auth/dataportability.saved.collections
    AuthDataportabilitySavedCollections,
    
    /// Documentation: Move a copy of your shipping information on Shopping., Scope: https://www.googleapis.com/auth/dataportability.shopping.addresses
    AuthDataportabilityShoppingAddresses,
    
    /// Documentation: Move a copy of reviews you wrote about products or online stores on Google Search., Scope: https://www.googleapis.com/auth/dataportability.shopping.reviews
    AuthDataportabilityShoppingReviews,
    
    /// Documentation: Move a copy of information about your YouTube channel., Scope: https://www.googleapis.com/auth/dataportability.youtube.channel
    AuthDataportabilityYoutubeChannel,
    
    /// Documentation: Move a copy of your YouTube comments., Scope: https://www.googleapis.com/auth/dataportability.youtube.comments
    AuthDataportabilityYoutubeComments,
    
    /// Documentation: Move a copy of your YouTube messages in live chat., Scope: https://www.googleapis.com/auth/dataportability.youtube.live_chat
    AuthDataportabilityYoutubeLive_Chat,
    
    /// Documentation: Move a copy of your uploaded YouTube music tracks and your YouTube music library., Scope: https://www.googleapis.com/auth/dataportability.youtube.music
    AuthDataportabilityYoutubeMusic,
    
    /// Documentation: Move a copy of your YouTube playables saved game progress files., Scope: https://www.googleapis.com/auth/dataportability.youtube.playable
    AuthDataportabilityYoutubePlayable,
    
    /// Documentation: Move a copy of your YouTube posts., Scope: https://www.googleapis.com/auth/dataportability.youtube.posts
    AuthDataportabilityYoutubePosts,
    
    /// Documentation: Move a copy of your YouTube private playlists., Scope: https://www.googleapis.com/auth/dataportability.youtube.private_playlists
    AuthDataportabilityYoutubePrivate_Playlists,
    
    /// Documentation: Move a copy of your private YouTube videos and information about them., Scope: https://www.googleapis.com/auth/dataportability.youtube.private_videos
    AuthDataportabilityYoutubePrivate_Videos,
    
    /// Documentation: Move a copy of your public YouTube playlists., Scope: https://www.googleapis.com/auth/dataportability.youtube.public_playlists
    AuthDataportabilityYoutubePublic_Playlists,
    
    /// Documentation: Move a copy of your public YouTube videos and information about them., Scope: https://www.googleapis.com/auth/dataportability.youtube.public_videos
    AuthDataportabilityYoutubePublic_Videos,
    
    /// Documentation: Move a copy of your YouTube shopping wishlists, and wishlist items., Scope: https://www.googleapis.com/auth/dataportability.youtube.shopping
    AuthDataportabilityYoutubeShopping,
    
    /// Documentation: Move a copy of your YouTube channel subscriptions, even if they're private., Scope: https://www.googleapis.com/auth/dataportability.youtube.subscriptions
    AuthDataportabilityYoutubeSubscriptions,
    
    /// Documentation: Move a copy of your unlisted YouTube playlists., Scope: https://www.googleapis.com/auth/dataportability.youtube.unlisted_playlists
    AuthDataportabilityYoutubeUnlisted_Playlists,
    
    /// Documentation: Move a copy of your unlisted YouTube videos and information about them., Scope: https://www.googleapis.com/auth/dataportability.youtube.unlisted_videos
    AuthDataportabilityYoutubeUnlisted_Videos,
    
}



pub enum DataflowAPIv1b3 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage your Google Compute Engine resources, Scope: https://www.googleapis.com/auth/compute
    AuthCompute,
    
    /// Documentation: View your Google Compute Engine resources, Scope: https://www.googleapis.com/auth/compute.readonly
    AuthComputeReadOnly,
    
    /// Documentation: See your primary Google Account email address, Scope: https://www.googleapis.com/auth/userinfo.email
    AuthUserinfoEmail,
    
}



pub enum DriveActivityAPIv2 {
    
    /// Documentation: View and add to the activity record of files in your Google Drive, Scope: https://www.googleapis.com/auth/drive.activity
    AuthDriveActivity,
    
    /// Documentation: View the activity record of files in your Google Drive, Scope: https://www.googleapis.com/auth/drive.activity.readonly
    AuthDriveActivityReadOnly,
    
}



pub enum EnterpriseLicenseManagerAPIv1 {
    
    /// Documentation: View and manage G Suite licenses for your domain, Scope: https://www.googleapis.com/auth/apps.licensing
    AuthAppsLicensing,
    
}



pub enum ErrorReportingAPIv1beta1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum FactCheckToolsAPIv1alpha1 {
    
    /// Documentation: See your primary Google Account email address, Scope: https://www.googleapis.com/auth/userinfo.email
    AuthUserinfoEmail,
    
}



pub enum FirebaseCloudMessagingAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Send messages and manage messaging subscriptions for your Firebase applications, Scope: https://www.googleapis.com/auth/firebase.messaging
    AuthFirebaseMessaging,
    
}



pub enum FirebaseDynamicLinksAPIv1 {
    
    /// Documentation: View and administer all your Firebase data and settings, Scope: https://www.googleapis.com/auth/firebase
    AuthFirebase,
    
}



pub enum FirebaseManagementAPIv1beta1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: View and administer all your Firebase data and settings, Scope: https://www.googleapis.com/auth/firebase
    AuthFirebase,
    
    /// Documentation: View all your Firebase data and settings, Scope: https://www.googleapis.com/auth/firebase.readonly
    AuthFirebaseReadOnly,
    
}



pub enum FirebaseRulesAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and administer all your Firebase data and settings, Scope: https://www.googleapis.com/auth/firebase
    AuthFirebase,
    
    /// Documentation: View all your Firebase data and settings, Scope: https://www.googleapis.com/auth/firebase.readonly
    AuthFirebaseReadOnly,
    
}



pub enum FitnessAPIv1 {
    
    /// Documentation: Use Google Fit to see and store your physical activity data, Scope: https://www.googleapis.com/auth/fitness.activity.read
    AuthFitnessActivityRead,
    
    /// Documentation: Add to your Google Fit physical activity data, Scope: https://www.googleapis.com/auth/fitness.activity.write
    AuthFitnessActivityWrite,
    
    /// Documentation: See info about your blood glucose in Google Fit. I consent to Google sharing my blood glucose information with this app., Scope: https://www.googleapis.com/auth/fitness.blood_glucose.read
    AuthFitnessBlood_GlucoseRead,
    
    /// Documentation: Add info about your blood glucose to Google Fit. I consent to Google using my blood glucose information with this app., Scope: https://www.googleapis.com/auth/fitness.blood_glucose.write
    AuthFitnessBlood_GlucoseWrite,
    
    /// Documentation: See info about your blood pressure in Google Fit. I consent to Google sharing my blood pressure information with this app., Scope: https://www.googleapis.com/auth/fitness.blood_pressure.read
    AuthFitnessBlood_PressureRead,
    
    /// Documentation: Add info about your blood pressure in Google Fit. I consent to Google using my blood pressure information with this app., Scope: https://www.googleapis.com/auth/fitness.blood_pressure.write
    AuthFitnessBlood_PressureWrite,
    
    /// Documentation: See info about your body measurements in Google Fit, Scope: https://www.googleapis.com/auth/fitness.body.read
    AuthFitnessBodyRead,
    
    /// Documentation: Add info about your body measurements to Google Fit, Scope: https://www.googleapis.com/auth/fitness.body.write
    AuthFitnessBodyWrite,
    
    /// Documentation: See info about your body temperature in Google Fit. I consent to Google sharing my body temperature information with this app., Scope: https://www.googleapis.com/auth/fitness.body_temperature.read
    AuthFitnessBody_TemperatureRead,
    
    /// Documentation: Add to info about your body temperature in Google Fit. I consent to Google using my body temperature information with this app., Scope: https://www.googleapis.com/auth/fitness.body_temperature.write
    AuthFitnessBody_TemperatureWrite,
    
    /// Documentation: See your heart rate data in Google Fit. I consent to Google sharing my heart rate information with this app., Scope: https://www.googleapis.com/auth/fitness.heart_rate.read
    AuthFitnessHeart_RateRead,
    
    /// Documentation: Add to your heart rate data in Google Fit. I consent to Google using my heart rate information with this app., Scope: https://www.googleapis.com/auth/fitness.heart_rate.write
    AuthFitnessHeart_RateWrite,
    
    /// Documentation: See your Google Fit speed and distance data, Scope: https://www.googleapis.com/auth/fitness.location.read
    AuthFitnessLocationRead,
    
    /// Documentation: Add to your Google Fit location data, Scope: https://www.googleapis.com/auth/fitness.location.write
    AuthFitnessLocationWrite,
    
    /// Documentation: See info about your nutrition in Google Fit, Scope: https://www.googleapis.com/auth/fitness.nutrition.read
    AuthFitnessNutritionRead,
    
    /// Documentation: Add to info about your nutrition in Google Fit, Scope: https://www.googleapis.com/auth/fitness.nutrition.write
    AuthFitnessNutritionWrite,
    
    /// Documentation: See info about your oxygen saturation in Google Fit. I consent to Google sharing my oxygen saturation information with this app., Scope: https://www.googleapis.com/auth/fitness.oxygen_saturation.read
    AuthFitnessOxygen_SaturationRead,
    
    /// Documentation: Add info about your oxygen saturation in Google Fit. I consent to Google using my oxygen saturation information with this app., Scope: https://www.googleapis.com/auth/fitness.oxygen_saturation.write
    AuthFitnessOxygen_SaturationWrite,
    
    /// Documentation: See info about your reproductive health in Google Fit. I consent to Google sharing my reproductive health information with this app., Scope: https://www.googleapis.com/auth/fitness.reproductive_health.read
    AuthFitnessReproductive_HealthRead,
    
    /// Documentation: Add info about your reproductive health in Google Fit. I consent to Google using my reproductive health information with this app., Scope: https://www.googleapis.com/auth/fitness.reproductive_health.write
    AuthFitnessReproductive_HealthWrite,
    
    /// Documentation: See your sleep data in Google Fit. I consent to Google sharing my sleep information with this app., Scope: https://www.googleapis.com/auth/fitness.sleep.read
    AuthFitnessSleepRead,
    
    /// Documentation: Add to your sleep data in Google Fit. I consent to Google using my sleep information with this app., Scope: https://www.googleapis.com/auth/fitness.sleep.write
    AuthFitnessSleepWrite,
    
}



pub enum GenomicsAPIv2alpha1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and manage Genomics data, Scope: https://www.googleapis.com/auth/genomics
    AuthGenomics,
    
}



pub enum GmailAPIv1 {
    
    /// Documentation: Read, compose, send, and permanently delete all your email from Gmail, Scope: https://mail.google.com/
    /,
    
    /// Documentation: Manage drafts and send emails when you interact with the add-on, Scope: https://www.googleapis.com/auth/gmail.addons.current.action.compose
    AuthGmailAddonsCurrentActionCompose,
    
    /// Documentation: View your email messages when you interact with the add-on, Scope: https://www.googleapis.com/auth/gmail.addons.current.message.action
    AuthGmailAddonsCurrentMessageAction,
    
    /// Documentation: View your email message metadata when the add-on is running, Scope: https://www.googleapis.com/auth/gmail.addons.current.message.metadata
    AuthGmailAddonsCurrentMessageMetadata,
    
    /// Documentation: View your email messages when the add-on is running, Scope: https://www.googleapis.com/auth/gmail.addons.current.message.readonly
    AuthGmailAddonsCurrentMessageReadOnly,
    
    /// Documentation: Manage drafts and send emails, Scope: https://www.googleapis.com/auth/gmail.compose
    AuthGmailCompose,
    
    /// Documentation: Add emails into your Gmail mailbox, Scope: https://www.googleapis.com/auth/gmail.insert
    AuthGmailInsert,
    
    /// Documentation: See and edit your email labels, Scope: https://www.googleapis.com/auth/gmail.labels
    AuthGmailLabels,
    
    /// Documentation: View your email message metadata such as labels and headers, but not the email body, Scope: https://www.googleapis.com/auth/gmail.metadata
    AuthGmailMetadata,
    
    /// Documentation: Read, compose, and send emails from your Gmail account, Scope: https://www.googleapis.com/auth/gmail.modify
    AuthGmailModify,
    
    /// Documentation: View your email messages and settings, Scope: https://www.googleapis.com/auth/gmail.readonly
    AuthGmailReadOnly,
    
    /// Documentation: Send email on your behalf, Scope: https://www.googleapis.com/auth/gmail.send
    AuthGmailSend,
    
    /// Documentation: See, edit, create, or change your email settings and filters in Gmail, Scope: https://www.googleapis.com/auth/gmail.settings.basic
    AuthGmailSettingsBasic,
    
    /// Documentation: Manage your sensitive mail settings, including who can manage your mail, Scope: https://www.googleapis.com/auth/gmail.settings.sharing
    AuthGmailSettingsSharing,
    
}



pub enum GoogleAnalyticsAPIv3 {
    
    /// Documentation: View and manage your Google Analytics data, Scope: https://www.googleapis.com/auth/analytics
    AuthAnalytics,
    
    /// Documentation: Edit Google Analytics management entities, Scope: https://www.googleapis.com/auth/analytics.edit
    AuthAnalyticsEdit,
    
    /// Documentation: Manage Google Analytics Account users by email address, Scope: https://www.googleapis.com/auth/analytics.manage.users
    AuthAnalyticsManageUsers,
    
    /// Documentation: View Google Analytics user permissions, Scope: https://www.googleapis.com/auth/analytics.manage.users.readonly
    AuthAnalyticsManageUsersReadOnly,
    
    /// Documentation: Create a new Google Analytics account along with its default property and view, Scope: https://www.googleapis.com/auth/analytics.provision
    AuthAnalyticsProvision,
    
    /// Documentation: View your Google Analytics data, Scope: https://www.googleapis.com/auth/analytics.readonly
    AuthAnalyticsReadOnly,
    
    /// Documentation: Manage Google Analytics user deletion requests, Scope: https://www.googleapis.com/auth/analytics.user.deletion
    AuthAnalyticsUserDeletion,
    
}



pub enum GoogleChatAPIv1 {
    
    /// Documentation: Delete conversations and spaces & remove access to associated files in Google Chat, Scope: https://www.googleapis.com/auth/chat.delete
    AuthChatDelete,
    
    /// Documentation: View, add, and remove members from conversations in Google Chat, Scope: https://www.googleapis.com/auth/chat.memberships
    AuthChatMemberships,
    
    /// Documentation: Add and remove itself from conversations in Google Chat, Scope: https://www.googleapis.com/auth/chat.memberships.app
    AuthChatMembershipsApp,
    
    /// Documentation: View members in Google Chat conversations., Scope: https://www.googleapis.com/auth/chat.memberships.readonly
    AuthChatMembershipsReadOnly,
    
    /// Documentation: View, compose, send, update, and delete messages, and add, view, and delete reactions to messages., Scope: https://www.googleapis.com/auth/chat.messages
    AuthChatMessages,
    
    /// Documentation: Compose and send messages in Google Chat, Scope: https://www.googleapis.com/auth/chat.messages.create
    AuthChatMessagesCreate,
    
    /// Documentation: View, add, and delete reactions to messages in Google Chat, Scope: https://www.googleapis.com/auth/chat.messages.reactions
    AuthChatMessagesReactions,
    
    /// Documentation: Add reactions to messages in Google Chat, Scope: https://www.googleapis.com/auth/chat.messages.reactions.create
    AuthChatMessagesReactionsCreate,
    
    /// Documentation: View reactions to messages in Google Chat, Scope: https://www.googleapis.com/auth/chat.messages.reactions.readonly
    AuthChatMessagesReactionsReadOnly,
    
    /// Documentation: View messages and reactions in Google Chat, Scope: https://www.googleapis.com/auth/chat.messages.readonly
    AuthChatMessagesReadOnly,
    
    /// Documentation: Create conversations and spaces and view or update metadata (including history settings) in Google Chat, Scope: https://www.googleapis.com/auth/chat.spaces
    AuthChatSpaces,
    
    /// Documentation: Create new conversations in Google Chat, Scope: https://www.googleapis.com/auth/chat.spaces.create
    AuthChatSpacesCreate,
    
    /// Documentation: View chat and spaces in Google Chat, Scope: https://www.googleapis.com/auth/chat.spaces.readonly
    AuthChatSpacesReadOnly,
    
}



pub enum GoogleClassroomAPIv1 {
    
    /// Documentation: View and manage announcements in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.announcements
    AuthClassroomAnnouncements,
    
    /// Documentation: View announcements in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.announcements.readonly
    AuthClassroomAnnouncementsReadOnly,
    
    /// Documentation: See, edit, create, and permanently delete your Google Classroom classes, Scope: https://www.googleapis.com/auth/classroom.courses
    AuthClassroomCourses,
    
    /// Documentation: View your Google Classroom classes, Scope: https://www.googleapis.com/auth/classroom.courses.readonly
    AuthClassroomCoursesReadOnly,
    
    /// Documentation: See, create and edit coursework items including assignments, questions, and grades, Scope: https://www.googleapis.com/auth/classroom.coursework.me
    AuthClassroomCourseworkMe,
    
    /// Documentation: View your course work and grades in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.coursework.me.readonly
    AuthClassroomCourseworkMeReadOnly,
    
    /// Documentation: Manage course work and grades for students in the Google Classroom classes you teach and view the course work and grades for classes you administer, Scope: https://www.googleapis.com/auth/classroom.coursework.students
    AuthClassroomCourseworkStudents,
    
    /// Documentation: View course work and grades for students in the Google Classroom classes you teach or administer, Scope: https://www.googleapis.com/auth/classroom.coursework.students.readonly
    AuthClassroomCourseworkStudentsReadOnly,
    
    /// Documentation: See, edit, and create classwork materials in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.courseworkmaterials
    AuthClassroomCourseworkmaterials,
    
    /// Documentation: See all classwork materials for your Google Classroom classes, Scope: https://www.googleapis.com/auth/classroom.courseworkmaterials.readonly
    AuthClassroomCourseworkmaterialsReadOnly,
    
    /// Documentation: View your Google Classroom guardians, Scope: https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly
    AuthClassroomGuardianlinksMeReadOnly,
    
    /// Documentation: View and manage guardians for students in your Google Classroom classes, Scope: https://www.googleapis.com/auth/classroom.guardianlinks.students
    AuthClassroomGuardianlinksStudents,
    
    /// Documentation: View guardians for students in your Google Classroom classes, Scope: https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly
    AuthClassroomGuardianlinksStudentsReadOnly,
    
    /// Documentation: View the email addresses of people in your classes, Scope: https://www.googleapis.com/auth/classroom.profile.emails
    AuthClassroomProfileEmails,
    
    /// Documentation: View the profile photos of people in your classes, Scope: https://www.googleapis.com/auth/classroom.profile.photos
    AuthClassroomProfilePhotos,
    
    /// Documentation: Receive notifications about your Google Classroom data, Scope: https://www.googleapis.com/auth/classroom.push-notifications
    AuthClassroomPushNotifications,
    
    /// Documentation: Manage your Google Classroom class rosters, Scope: https://www.googleapis.com/auth/classroom.rosters
    AuthClassroomRosters,
    
    /// Documentation: View your Google Classroom class rosters, Scope: https://www.googleapis.com/auth/classroom.rosters.readonly
    AuthClassroomRostersReadOnly,
    
    /// Documentation: View your course work and grades in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.student-submissions.me.readonly
    AuthClassroomStudentSubmissionsMeReadOnly,
    
    /// Documentation: View course work and grades for students in the Google Classroom classes you teach or administer, Scope: https://www.googleapis.com/auth/classroom.student-submissions.students.readonly
    AuthClassroomStudentSubmissionsStudentsReadOnly,
    
    /// Documentation: See, create, and edit topics in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.topics
    AuthClassroomTopics,
    
    /// Documentation: View topics in Google Classroom, Scope: https://www.googleapis.com/auth/classroom.topics.readonly
    AuthClassroomTopicsReadOnly,
    
}



pub enum GoogleCloudDataCatalogAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum GoogleCloudMemorystoreforRedisAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum GoogleDocsAPIv1 {
    
    /// Documentation: See, edit, create, and delete all your Google Docs documents, Scope: https://www.googleapis.com/auth/documents
    AuthDocuments,
    
    /// Documentation: See all your Google Docs documents, Scope: https://www.googleapis.com/auth/documents.readonly
    AuthDocumentsReadOnly,
    
    /// Documentation: See, edit, create, and delete all of your Google Drive files, Scope: https://www.googleapis.com/auth/drive
    AuthDrive,
    
    /// Documentation: See, edit, create, and delete only the specific Google Drive files you use with this app, Scope: https://www.googleapis.com/auth/drive.file
    AuthDriveFile,
    
    /// Documentation: See and download all your Google Drive files, Scope: https://www.googleapis.com/auth/drive.readonly
    AuthDriveReadOnly,
    
}



pub enum GoogleDriveAPIv3 {
    
    /// Documentation: See, edit, create, and delete all of your Google Drive files, Scope: https://www.googleapis.com/auth/drive
    AuthDrive,
    
    /// Documentation: See, create, and delete its own configuration data in your Google Drive, Scope: https://www.googleapis.com/auth/drive.appdata
    AuthDriveAppdata,
    
    /// Documentation: See, edit, create, and delete only the specific Google Drive files you use with this app, Scope: https://www.googleapis.com/auth/drive.file
    AuthDriveFile,
    
    /// Documentation: View and manage metadata of files in your Google Drive, Scope: https://www.googleapis.com/auth/drive.metadata
    AuthDriveMetadata,
    
    /// Documentation: See information about your Google Drive files, Scope: https://www.googleapis.com/auth/drive.metadata.readonly
    AuthDriveMetadataReadOnly,
    
    /// Documentation: View the photos, videos and albums in your Google Photos, Scope: https://www.googleapis.com/auth/drive.photos.readonly
    AuthDrivePhotosReadOnly,
    
    /// Documentation: See and download all your Google Drive files, Scope: https://www.googleapis.com/auth/drive.readonly
    AuthDriveReadOnly,
    
    /// Documentation: Modify your Google Apps Script scripts' behavior, Scope: https://www.googleapis.com/auth/drive.scripts
    AuthDriveScripts,
    
}



pub enum GoogleIdentityToolkitAPIv3 {
    
    /// Documentation: View and manage your data across Google Cloud Platform services, Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View and administer all your Firebase data and settings, Scope: https://www.googleapis.com/auth/firebase
    AuthFirebase,
    
}



pub enum GoogleOAuth2APIv2 {
    
    /// Documentation: See your primary Google Account email address, Scope: https://www.googleapis.com/auth/userinfo.email
    AuthUserinfoEmail,
    
    /// Documentation: See your personal info, including any personal info you've made publicly available, Scope: https://www.googleapis.com/auth/userinfo.profile
    AuthUserinfoProfile,
    
}



pub enum GooglePlayAndroidDeveloperAPIv3 {
    
    /// Documentation: View and manage your Google Play Developer account, Scope: https://www.googleapis.com/auth/androidpublisher
    AuthAndroidpublisher,
    
}



pub enum GooglePlayCustomAppPublishingAPIv1 {
    
    /// Documentation: View and manage your Google Play Developer account, Scope: https://www.googleapis.com/auth/androidpublisher
    AuthAndroidpublisher,
    
}



pub enum GooglePlayEMMAPIv1 {
    
    /// Documentation: Manage corporate Android devices, Scope: https://www.googleapis.com/auth/androidenterprise
    AuthAndroidenterprise,
    
    /// Documentation: Create, edit, and delete your Google Play Games activity, Scope: https://www.googleapis.com/auth/games
    AuthGames,
    
    /// Documentation: See, create, and delete its own configuration data in your Google Drive, Scope: https://www.googleapis.com/auth/drive.appdata
    AuthDriveAppdata,
    
    /// Documentation: Create, edit, and delete your Google Play Games activity, Scope: https://www.googleapis.com/auth/games
    AuthGames,
    
}



pub enum GooglePlayGameServicesPublishingAPIv1configuration {
    
    /// Documentation: View and manage your Google Play Developer account, Scope: https://www.googleapis.com/auth/androidpublisher
    AuthAndroidpublisher,
    
}



pub enum GoogleSearchConsoleAPIv1 {
    
    /// Documentation: View and manage Search Console data for your verified sites, Scope: https://www.googleapis.com/auth/webmasters
    AuthWebmasters,
    
    /// Documentation: View Search Console data for your verified sites, Scope: https://www.googleapis.com/auth/webmasters.readonly
    AuthWebmastersReadOnly,
    
}



pub enum GoogleSheetsAPIv4 {
    
    /// Documentation: See, edit, create, and delete all of your Google Drive files, Scope: https://www.googleapis.com/auth/drive
    AuthDrive,
    
    /// Documentation: See, edit, create, and delete only the specific Google Drive files you use with this app, Scope: https://www.googleapis.com/auth/drive.file
    AuthDriveFile,
    
    /// Documentation: See and download all your Google Drive files, Scope: https://www.googleapis.com/auth/drive.readonly
    AuthDriveReadOnly,
    
    /// Documentation: See, edit, create, and delete all your Google Sheets spreadsheets, Scope: https://www.googleapis.com/auth/spreadsheets
    AuthSpreadsheets,
    
    /// Documentation: See all your Google Sheets spreadsheets, Scope: https://www.googleapis.com/auth/spreadsheets.readonly
    AuthSpreadsheetsReadOnly,
    
}



pub enum GoogleSiteVerificationAPIv1 {
    
    /// Documentation: Manage the list of sites and domains you control, Scope: https://www.googleapis.com/auth/siteverification
    AuthSiteverification,
    
    /// Documentation: Manage your new site verifications with Google, Scope: https://www.googleapis.com/auth/siteverification.verify_only
    AuthSiteverificationVerify_Only,
    
}



pub enum GoogleSlidesAPIv1 {
    
    /// Documentation: See, edit, create, and delete all of your Google Drive files, Scope: https://www.googleapis.com/auth/drive
    AuthDrive,
    
    /// Documentation: See, edit, create, and delete only the specific Google Drive files you use with this app, Scope: https://www.googleapis.com/auth/drive.file
    AuthDriveFile,
    
    /// Documentation: See and download all your Google Drive files, Scope: https://www.googleapis.com/auth/drive.readonly
    AuthDriveReadOnly,
    
    /// Documentation: See, edit, create, and delete all your Google Slides presentations, Scope: https://www.googleapis.com/auth/presentations
    AuthPresentations,
    
    /// Documentation: See all your Google Slides presentations, Scope: https://www.googleapis.com/auth/presentations.readonly
    AuthPresentationsReadOnly,
    
    /// Documentation: See, edit, create, and delete all your Google Sheets spreadsheets, Scope: https://www.googleapis.com/auth/spreadsheets
    AuthSpreadsheets,
    
    /// Documentation: See all your Google Sheets spreadsheets, Scope: https://www.googleapis.com/auth/spreadsheets.readonly
    AuthSpreadsheetsReadOnly,
    
}



pub enum GoogleTasksAPIv1 {
    
    /// Documentation: Create, edit, organize, and delete all your tasks, Scope: https://www.googleapis.com/auth/tasks
    AuthTasks,
    
    /// Documentation: View your tasks, Scope: https://www.googleapis.com/auth/tasks.readonly
    AuthTasksReadOnly,
    
}



pub enum GoogleVaultAPIv1 {
    
    /// Documentation: Manage your eDiscovery data, Scope: https://www.googleapis.com/auth/ediscovery
    AuthEdiscovery,
    
    /// Documentation: View your eDiscovery data, Scope: https://www.googleapis.com/auth/ediscovery.readonly
    AuthEdiscoveryReadOnly,
    
}



pub enum GoogleWorkspaceAlertCenterAPIv1beta1 {
    
    /// Documentation: See and delete your domain's G Suite alerts, and send alert feedback, Scope: https://www.googleapis.com/auth/apps.alerts
    AuthAppsAlerts,
    
}



pub enum GoogleWorkspaceResellerAPIv1 {
    
    /// Documentation: Manage users on your domain, Scope: https://www.googleapis.com/auth/apps.order
    AuthAppsOrder,
    
    /// Documentation: Manage users on your domain, Scope: https://www.googleapis.com/auth/apps.order.readonly
    AuthAppsOrderReadOnly,
    
}



pub enum GroupsMigrationAPIv1 {
    
    /// Documentation: Upload messages to any Google group in your domain, Scope: https://www.googleapis.com/auth/apps.groups.migration
    AuthAppsGroupsMigration,
    
}



pub enum GroupsSettingsAPIv1 {
    
    /// Documentation: View and manage the settings of a G Suite group, Scope: https://www.googleapis.com/auth/apps.groups.settings
    AuthAppsGroupsSettings,
    
}



pub enum IAMServiceAccountCredentialsAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum IndexingAPIv3 {
    
    /// Documentation: Submit data to Google for indexing, Scope: https://www.googleapis.com/auth/indexing
    AuthIndexing,
    
}



pub enum KubernetesEngineAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum LibraryAgentAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum ManagedServiceforMicrosoftActiveDirectoryAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum ManufacturerCenterAPIv1 {
    
    /// Documentation: Manage your product listings for Google Manufacturer Center, Scope: https://www.googleapis.com/auth/manufacturercenter
    AuthManufacturercenter,
    
}



pub enum NetworkManagementAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum OSConfigAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum PeopleAPIv1 {
    
    /// Documentation: See, edit, download, and permanently delete your contacts, Scope: https://www.googleapis.com/auth/contacts
    AuthContacts,
    
    /// Documentation: See and download contact info automatically saved in your "Other contacts", Scope: https://www.googleapis.com/auth/contacts.other.readonly
    AuthContactsOtherReadOnly,
    
    /// Documentation: See and download your contacts, Scope: https://www.googleapis.com/auth/contacts.readonly
    AuthContactsReadOnly,
    
    /// Documentation: See and download your organization's GSuite directory, Scope: https://www.googleapis.com/auth/directory.readonly
    AuthDirectoryReadOnly,
    
    /// Documentation: View your street addresses, Scope: https://www.googleapis.com/auth/user.addresses.read
    AuthUserAddressesRead,
    
    /// Documentation: See and download your exact date of birth, Scope: https://www.googleapis.com/auth/user.birthday.read
    AuthUserBirthdayRead,
    
    /// Documentation: See and download all of your Google Account email addresses, Scope: https://www.googleapis.com/auth/user.emails.read
    AuthUserEmailsRead,
    
    /// Documentation: See your gender, Scope: https://www.googleapis.com/auth/user.gender.read
    AuthUserGenderRead,
    
    /// Documentation: See your education, work history and org info, Scope: https://www.googleapis.com/auth/user.organization.read
    AuthUserOrganizationRead,
    
    /// Documentation: See and download your personal phone numbers, Scope: https://www.googleapis.com/auth/user.phonenumbers.read
    AuthUserPhonenumbersRead,
    
    /// Documentation: See your primary Google Account email address, Scope: https://www.googleapis.com/auth/userinfo.email
    AuthUserinfoEmail,
    
    /// Documentation: See your personal info, including any personal info you've made publicly available, Scope: https://www.googleapis.com/auth/userinfo.profile
    AuthUserinfoProfile,
    
}



pub enum PhotosLibraryAPIv1 {
    
    /// Documentation: See, upload, and organize items in your Google Photos library, Scope: https://www.googleapis.com/auth/photoslibrary
    AuthPhotoslibrary,
    
    /// Documentation: Add to your Google Photos library, Scope: https://www.googleapis.com/auth/photoslibrary.appendonly
    AuthPhotoslibraryAppendOnly,
    
    /// Documentation: Edit the info in your photos, videos, and albums created within this app, including titles, descriptions, and covers, Scope: https://www.googleapis.com/auth/photoslibrary.edit.appcreateddata
    AuthPhotoslibraryEditAppcreateddata,
    
    /// Documentation: View your Google Photos library, Scope: https://www.googleapis.com/auth/photoslibrary.readonly
    AuthPhotoslibraryReadOnly,
    
    /// Documentation: Manage photos added by this app, Scope: https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata
    AuthPhotoslibraryReadOnlyAppcreateddata,
    
    /// Documentation: Manage and add to shared albums on your behalf, Scope: https://www.googleapis.com/auth/photoslibrary.sharing
    AuthPhotoslibrarySharing,
    
}



pub enum PolicyTroubleshooterAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum RecommenderAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum SASPortalAPI(Testing)v1alpha1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Read, create, update, and delete your SAS Portal data., Scope: https://www.googleapis.com/auth/sasportal
    AuthSasportal,
    
}



pub enum SASPortalAPIv1alpha1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Read, create, update, and delete your SAS Portal data., Scope: https://www.googleapis.com/auth/sasportal
    AuthSasportal,
    
}



pub enum SearchAds360APIv2 {
    
    /// Documentation: View and manage your advertising data in DoubleClick Search, Scope: https://www.googleapis.com/auth/doubleclicksearch
    AuthDoubleclicksearch,
    
}



pub enum SecretManagerAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum SecurityCommandCenterAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum ServerlessVPCAccessAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum ServiceConsumerManagementAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum ServiceManagementAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: Manage your Google API service configuration, Scope: https://www.googleapis.com/auth/service.management
    AuthServiceManagement,
    
    /// Documentation: View your Google API service configuration, Scope: https://www.googleapis.com/auth/service.management.readonly
    AuthServiceManagementReadOnly,
    
}



pub enum ServiceNetworkingAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: Manage your Google API service configuration, Scope: https://www.googleapis.com/auth/service.management
    AuthServiceManagement,
    
}



pub enum ServiceUsageAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
    /// Documentation: View your data across Google Cloud services and see the email address of your Google Account, Scope: https://www.googleapis.com/auth/cloud-platform.read-only
    AuthCloudPlatformReadOnly,
    
    /// Documentation: Manage your Google API service configuration, Scope: https://www.googleapis.com/auth/service.management
    AuthServiceManagement,
    
}



pub enum StorageTransferAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum StreetViewPublishAPIv1 {
    
    /// Documentation: Publish and manage your 360 photos on Google Street View, Scope: https://www.googleapis.com/auth/streetviewpublish
    AuthStreetviewpublish,
    
}



pub enum TagManagerAPIv2 {
    
    /// Documentation: Delete your Google Tag Manager containers, Scope: https://www.googleapis.com/auth/tagmanager.delete.containers
    AuthTagmanagerDeleteContainers,
    
    /// Documentation: Manage your Google Tag Manager container and its subcomponents, excluding versioning and publishing, Scope: https://www.googleapis.com/auth/tagmanager.edit.containers
    AuthTagmanagerEditContainers,
    
    /// Documentation: Manage your Google Tag Manager container versions, Scope: https://www.googleapis.com/auth/tagmanager.edit.containerversions
    AuthTagmanagerEditContainerversions,
    
    /// Documentation: View and manage your Google Tag Manager accounts, Scope: https://www.googleapis.com/auth/tagmanager.manage.accounts
    AuthTagmanagerManageAccounts,
    
    /// Documentation: Manage user permissions of your Google Tag Manager account and container, Scope: https://www.googleapis.com/auth/tagmanager.manage.users
    AuthTagmanagerManageUsers,
    
    /// Documentation: Publish your Google Tag Manager container versions, Scope: https://www.googleapis.com/auth/tagmanager.publish
    AuthTagmanagerPublish,
    
    /// Documentation: View your Google Tag Manager container and its subcomponents, Scope: https://www.googleapis.com/auth/tagmanager.readonly
    AuthTagmanagerReadOnly,
    
}



pub enum WebSecurityScannerAPIv1 {
    
    /// Documentation: See, edit, configure, and delete your Google Cloud data and see the email address for your Google Account., Scope: https://www.googleapis.com/auth/cloud-platform
    AuthCloudPlatform,
    
}



pub enum YouTubeAnalyticsAPIv2 {
    
    /// Documentation: Manage your YouTube account, Scope: https://www.googleapis.com/auth/youtube
    AuthYoutube,
    
    /// Documentation: View your YouTube account, Scope: https://www.googleapis.com/auth/youtube.readonly
    AuthYoutubeReadOnly,
    
    /// Documentation: View and manage your assets and associated content on YouTube, Scope: https://www.googleapis.com/auth/youtubepartner
    AuthYoutubepartner,
    
    /// Documentation: View monetary and non-monetary YouTube Analytics reports for your YouTube content, Scope: https://www.googleapis.com/auth/yt-analytics-monetary.readonly
    AuthYtAnalyticsMonetaryReadOnly,
    
    /// Documentation: View YouTube Analytics reports for your YouTube content, Scope: https://www.googleapis.com/auth/yt-analytics.readonly
    AuthYtAnalyticsReadOnly,
    
}



pub enum YouTubeDataAPIv3v3 {
    
    /// Documentation: Manage your YouTube account, Scope: https://www.googleapis.com/auth/youtube
    AuthYoutube,
    
    /// Documentation: See a list of your current active channel members, their current level, and when they became a member, Scope: https://www.googleapis.com/auth/youtube.channel-memberships.creator
    AuthYoutubeChannelMembershipsCreator,
    
    /// Documentation: See, edit, and permanently delete your YouTube videos, ratings, comments and captions, Scope: https://www.googleapis.com/auth/youtube.force-ssl
    AuthYoutubeForceSsl,
    
    /// Documentation: View your YouTube account, Scope: https://www.googleapis.com/auth/youtube.readonly
    AuthYoutubeReadOnly,
    
    /// Documentation: Manage your YouTube videos, Scope: https://www.googleapis.com/auth/youtube.upload
    AuthYoutubeUpload,
    
    /// Documentation: View and manage your assets and associated content on YouTube, Scope: https://www.googleapis.com/auth/youtubepartner
    AuthYoutubepartner,
    
    /// Documentation: View private information of your YouTube channel relevant during the audit process with a YouTube partner, Scope: https://www.googleapis.com/auth/youtubepartner-channel-audit
    AuthYoutubepartnerChannelAudit,
    
}



pub enum YouTubeReportingAPIv1 {
    
    /// Documentation: View monetary and non-monetary YouTube Analytics reports for your YouTube content, Scope: https://www.googleapis.com/auth/yt-analytics-monetary.readonly
    AuthYtAnalyticsMonetaryReadOnly,
    
    /// Documentation: View YouTube Analytics reports for your YouTube content, Scope: https://www.googleapis.com/auth/yt-analytics.readonly
    AuthYtAnalyticsReadOnly,
    
}






impl ToGoogleScope for AIPlatformTrainingAndPredictionAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AIPlatformTrainingAndPredictionAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            AIPlatformTrainingAndPredictionAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            
        }
    }
}




impl ToGoogleScope for AccessApprovalAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AccessApprovalAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for AccessContextManagerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AccessContextManagerAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for AdExchangeBuyerAPIIIv2beta1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AdExchangeBuyerAPIIIv2beta1::AuthAdexchangeBuyer => "https://www.googleapis.com/auth/adexchange.buyer",
            
        }
    }
}




impl ToGoogleScope for AdMobAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AdMobAPIv1::AuthAdexchangeBuyer => "https://www.googleapis.com/auth/adexchange.buyer",
            AdMobAPIv1::AuthAdmobReport => "https://www.googleapis.com/auth/admob.report",
            
        }
    }
}




impl ToGoogleScope for AdSenseHostAPIv4Point1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AdSenseHostAPIv4Point1::AuthAdsensehost => "https://www.googleapis.com/auth/adsensehost",
            
        }
    }
}




impl ToGoogleScope for AdminSDKAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AdminSDKAPIv1::AuthAdminReportsAuditReadOnly => "https://www.googleapis.com/auth/admin.reports.audit.readonly",
            AdminSDKAPIv1::AuthAdminReportsUsageReadOnly => "https://www.googleapis.com/auth/admin.reports.usage.readonly",
            AdminSDKAPIv1::AuthAdminDatatransfer => "https://www.googleapis.com/auth/admin.datatransfer",
            AdminSDKAPIv1::AuthAdminDatatransferReadOnly => "https://www.googleapis.com/auth/admin.datatransfer.readonly",
            AdminSDKAPIv1::AuthAdminChromePrinters => "https://www.googleapis.com/auth/admin.chrome.printers",
            AdminSDKAPIv1::AuthAdminChromePrintersReadOnly => "https://www.googleapis.com/auth/admin.chrome.printers.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryCustomer => "https://www.googleapis.com/auth/admin.directory.customer",
            AdminSDKAPIv1::AuthAdminDirectoryCustomerReadOnly => "https://www.googleapis.com/auth/admin.directory.customer.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryDeviceChromeos => "https://www.googleapis.com/auth/admin.directory.device.chromeos",
            AdminSDKAPIv1::AuthAdminDirectoryDeviceChromeosReadOnly => "https://www.googleapis.com/auth/admin.directory.device.chromeos.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryDeviceMobile => "https://www.googleapis.com/auth/admin.directory.device.mobile",
            AdminSDKAPIv1::AuthAdminDirectoryDeviceMobileAction => "https://www.googleapis.com/auth/admin.directory.device.mobile.action",
            AdminSDKAPIv1::AuthAdminDirectoryDeviceMobileReadOnly => "https://www.googleapis.com/auth/admin.directory.device.mobile.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryDomain => "https://www.googleapis.com/auth/admin.directory.domain",
            AdminSDKAPIv1::AuthAdminDirectoryDomainReadOnly => "https://www.googleapis.com/auth/admin.directory.domain.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryGroup => "https://www.googleapis.com/auth/admin.directory.group",
            AdminSDKAPIv1::AuthAdminDirectoryGroupMember => "https://www.googleapis.com/auth/admin.directory.group.member",
            AdminSDKAPIv1::AuthAdminDirectoryGroupMemberReadOnly => "https://www.googleapis.com/auth/admin.directory.group.member.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryGroupReadOnly => "https://www.googleapis.com/auth/admin.directory.group.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryOrgunit => "https://www.googleapis.com/auth/admin.directory.orgunit",
            AdminSDKAPIv1::AuthAdminDirectoryOrgunitReadOnly => "https://www.googleapis.com/auth/admin.directory.orgunit.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryResourceCalendar => "https://www.googleapis.com/auth/admin.directory.resource.calendar",
            AdminSDKAPIv1::AuthAdminDirectoryResourceCalendarReadOnly => "https://www.googleapis.com/auth/admin.directory.resource.calendar.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryRolemanagement => "https://www.googleapis.com/auth/admin.directory.rolemanagement",
            AdminSDKAPIv1::AuthAdminDirectoryRolemanagementReadOnly => "https://www.googleapis.com/auth/admin.directory.rolemanagement.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryUser => "https://www.googleapis.com/auth/admin.directory.user",
            AdminSDKAPIv1::AuthAdminDirectoryUserAlias => "https://www.googleapis.com/auth/admin.directory.user.alias",
            AdminSDKAPIv1::AuthAdminDirectoryUserAliasReadOnly => "https://www.googleapis.com/auth/admin.directory.user.alias.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryUserReadOnly => "https://www.googleapis.com/auth/admin.directory.user.readonly",
            AdminSDKAPIv1::AuthAdminDirectoryUserSecurity => "https://www.googleapis.com/auth/admin.directory.user.security",
            AdminSDKAPIv1::AuthAdminDirectoryUserschema => "https://www.googleapis.com/auth/admin.directory.userschema",
            AdminSDKAPIv1::AuthAdminDirectoryUserschemaReadOnly => "https://www.googleapis.com/auth/admin.directory.userschema.readonly",
            AdminSDKAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for AnalyticsReportingAPIv4 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AnalyticsReportingAPIv4::AuthAnalytics => "https://www.googleapis.com/auth/analytics",
            AnalyticsReportingAPIv4::AuthAnalyticsReadOnly => "https://www.googleapis.com/auth/analytics.readonly",
            
        }
    }
}




impl ToGoogleScope for AndroidManagementAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AndroidManagementAPIv1::AuthAndroidmanagement => "https://www.googleapis.com/auth/androidmanagement",
            
        }
    }
}




impl ToGoogleScope for ApigeeAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ApigeeAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for AppEngineAdminAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AppEngineAdminAPIv1::AuthAppengineAdmin => "https://www.googleapis.com/auth/appengine.admin",
            AppEngineAdminAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            AppEngineAdminAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            
        }
    }
}




impl ToGoogleScope for AppsScriptAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            AppsScriptAPIv1::/ => "https://mail.google.com/",
            AppsScriptAPIv1::CalendarFeeds => "https://www.google.com/calendar/feeds",
            AppsScriptAPIv1::M8Feeds => "https://www.google.com/m8/feeds",
            AppsScriptAPIv1::AuthAdminDirectoryGroup => "https://www.googleapis.com/auth/admin.directory.group",
            AppsScriptAPIv1::AuthAdminDirectoryUser => "https://www.googleapis.com/auth/admin.directory.user",
            AppsScriptAPIv1::AuthDocuments => "https://www.googleapis.com/auth/documents",
            AppsScriptAPIv1::AuthDrive => "https://www.googleapis.com/auth/drive",
            AppsScriptAPIv1::AuthForms => "https://www.googleapis.com/auth/forms",
            AppsScriptAPIv1::AuthFormsCurrentOnly => "https://www.googleapis.com/auth/forms.currentonly",
            AppsScriptAPIv1::AuthGroups => "https://www.googleapis.com/auth/groups",
            AppsScriptAPIv1::AuthScriptDeployments => "https://www.googleapis.com/auth/script.deployments",
            AppsScriptAPIv1::AuthScriptDeploymentsReadOnly => "https://www.googleapis.com/auth/script.deployments.readonly",
            AppsScriptAPIv1::AuthScriptMetrics => "https://www.googleapis.com/auth/script.metrics",
            AppsScriptAPIv1::AuthScriptProcesses => "https://www.googleapis.com/auth/script.processes",
            AppsScriptAPIv1::AuthScriptProjects => "https://www.googleapis.com/auth/script.projects",
            AppsScriptAPIv1::AuthScriptProjectsReadOnly => "https://www.googleapis.com/auth/script.projects.readonly",
            AppsScriptAPIv1::AuthSpreadsheets => "https://www.googleapis.com/auth/spreadsheets",
            AppsScriptAPIv1::AuthUserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            
        }
    }
}




impl ToGoogleScope for BigQueryAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BigQueryAPIv2::AuthBigquery => "https://www.googleapis.com/auth/bigquery",
            BigQueryAPIv2::AuthBigqueryInsertdata => "https://www.googleapis.com/auth/bigquery.insertdata",
            BigQueryAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            BigQueryAPIv2::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            BigQueryAPIv2::AuthDevstorageFull_Control => "https://www.googleapis.com/auth/devstorage.full_control",
            BigQueryAPIv2::AuthDevstorageRead_Only => "https://www.googleapis.com/auth/devstorage.read_only",
            BigQueryAPIv2::AuthDevstorageRead_Write => "https://www.googleapis.com/auth/devstorage.read_write",
            
        }
    }
}




impl ToGoogleScope for BigQueryConnectionAPIv1beta1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BigQueryConnectionAPIv1beta1::AuthBigquery => "https://www.googleapis.com/auth/bigquery",
            BigQueryConnectionAPIv1beta1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for BigQueryDataTransferAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BigQueryDataTransferAPIv1::AuthBigquery => "https://www.googleapis.com/auth/bigquery",
            BigQueryDataTransferAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            BigQueryDataTransferAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            
        }
    }
}




impl ToGoogleScope for BigQueryReservationAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BigQueryReservationAPIv1::AuthBigquery => "https://www.googleapis.com/auth/bigquery",
            BigQueryReservationAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for BinaryAuthorizationAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BinaryAuthorizationAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for BloggerAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BloggerAPIv3::AuthBlogger => "https://www.googleapis.com/auth/blogger",
            BloggerAPIv3::AuthBloggerReadOnly => "https://www.googleapis.com/auth/blogger.readonly",
            
        }
    }
}




impl ToGoogleScope for BooksAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            BooksAPIv1::AuthBooks => "https://www.googleapis.com/auth/books",
            
        }
    }
}




impl ToGoogleScope for CalendarAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CalendarAPIv3::AuthCalendar => "https://www.googleapis.com/auth/calendar",
            CalendarAPIv3::AuthCalendarEvents => "https://www.googleapis.com/auth/calendar.events",
            CalendarAPIv3::AuthCalendarEventsReadOnly => "https://www.googleapis.com/auth/calendar.events.readonly",
            CalendarAPIv3::AuthCalendarReadOnly => "https://www.googleapis.com/auth/calendar.readonly",
            CalendarAPIv3::AuthCalendarSettingsReadOnly => "https://www.googleapis.com/auth/calendar.settings.readonly",
            
        }
    }
}




impl ToGoogleScope for CampaignManager360APIv4 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CampaignManager360APIv4::AuthDdmconversions => "https://www.googleapis.com/auth/ddmconversions",
            CampaignManager360APIv4::AuthDfareporting => "https://www.googleapis.com/auth/dfareporting",
            CampaignManager360APIv4::AuthDfatrafficking => "https://www.googleapis.com/auth/dfatrafficking",
            
        }
    }
}




impl ToGoogleScope for CloudAssetAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudAssetAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudBigtableAdminAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudBigtableAdminAPIv2::AuthBigtableAdmin => "https://www.googleapis.com/auth/bigtable.admin",
            CloudBigtableAdminAPIv2::AuthBigtableAdminCluster => "https://www.googleapis.com/auth/bigtable.admin.cluster",
            CloudBigtableAdminAPIv2::AuthBigtableAdminInstance => "https://www.googleapis.com/auth/bigtable.admin.instance",
            CloudBigtableAdminAPIv2::AuthBigtableAdminTable => "https://www.googleapis.com/auth/bigtable.admin.table",
            CloudBigtableAdminAPIv2::AuthCloudBigtableAdmin => "https://www.googleapis.com/auth/cloud-bigtable.admin",
            CloudBigtableAdminAPIv2::AuthCloudBigtableAdminCluster => "https://www.googleapis.com/auth/cloud-bigtable.admin.cluster",
            CloudBigtableAdminAPIv2::AuthCloudBigtableAdminTable => "https://www.googleapis.com/auth/cloud-bigtable.admin.table",
            CloudBigtableAdminAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudBigtableAdminAPIv2::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            
        }
    }
}




impl ToGoogleScope for CloudBillingAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudBillingAPIv1::AuthCloudBilling => "https://www.googleapis.com/auth/cloud-billing",
            CloudBillingAPIv1::AuthCloudBillingReadOnly => "https://www.googleapis.com/auth/cloud-billing.readonly",
            CloudBillingAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudBuildAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudBuildAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudComposerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudComposerAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudDNSAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudDNSAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudDNSAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            CloudDNSAPIv1::AuthNdevClouddnsReadOnly => "https://www.googleapis.com/auth/ndev.clouddns.readonly",
            CloudDNSAPIv1::AuthNdevClouddnsReadwrite => "https://www.googleapis.com/auth/ndev.clouddns.readwrite",
            
        }
    }
}




impl ToGoogleScope for CloudDataLossPrevention(DLP)APIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudDataLossPrevention(DLP)APIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudDataprocAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudDataprocAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudDatastoreAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudDatastoreAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudDatastoreAPIv1::AuthDatastore => "https://www.googleapis.com/auth/datastore",
            
        }
    }
}




impl ToGoogleScope for CloudDebuggerAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudDebuggerAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudDebuggerAPIv2::AuthCloud_Debugger => "https://www.googleapis.com/auth/cloud_debugger",
            
        }
    }
}




impl ToGoogleScope for CloudDeploymentManagerV2APIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudDeploymentManagerV2APIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudDeploymentManagerV2APIv2::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            CloudDeploymentManagerV2APIv2::AuthNdevCloudman => "https://www.googleapis.com/auth/ndev.cloudman",
            CloudDeploymentManagerV2APIv2::AuthNdevCloudmanReadOnly => "https://www.googleapis.com/auth/ndev.cloudman.readonly",
            
        }
    }
}




impl ToGoogleScope for CloudFilestoreAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudFilestoreAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudFirestoreAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudFirestoreAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudFirestoreAPIv1::AuthDatastore => "https://www.googleapis.com/auth/datastore",
            
        }
    }
}




impl ToGoogleScope for CloudHealthcareAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudHealthcareAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudIdentity-AwareProxyAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudIdentity-AwareProxyAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudIdentityAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudIdentityAPIv1::AuthCloudIdentityDevicesLookup => "https://www.googleapis.com/auth/cloud-identity.devices.lookup",
            CloudIdentityAPIv1::AuthCloudIdentityGroups => "https://www.googleapis.com/auth/cloud-identity.groups",
            CloudIdentityAPIv1::AuthCloudIdentityGroupsReadOnly => "https://www.googleapis.com/auth/cloud-identity.groups.readonly",
            CloudIdentityAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudKeyManagementService(KMS)APIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudKeyManagementService(KMS)APIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudKeyManagementService(KMS)APIv1::AuthCloudkms => "https://www.googleapis.com/auth/cloudkms",
            
        }
    }
}




impl ToGoogleScope for CloudLifeSciencesAPIv2beta {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudLifeSciencesAPIv2beta::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudLoggingAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudLoggingAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudLoggingAPIv2::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            CloudLoggingAPIv2::AuthLoggingAdmin => "https://www.googleapis.com/auth/logging.admin",
            CloudLoggingAPIv2::AuthLoggingRead => "https://www.googleapis.com/auth/logging.read",
            CloudLoggingAPIv2::AuthLoggingWrite => "https://www.googleapis.com/auth/logging.write",
            
        }
    }
}




impl ToGoogleScope for CloudMonitoringAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudMonitoringAPIv3::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudMonitoringAPIv3::AuthMonitoring => "https://www.googleapis.com/auth/monitoring",
            CloudMonitoringAPIv3::AuthMonitoringRead => "https://www.googleapis.com/auth/monitoring.read",
            CloudMonitoringAPIv3::AuthMonitoringWrite => "https://www.googleapis.com/auth/monitoring.write",
            
        }
    }
}




impl ToGoogleScope for CloudNaturalLanguageAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudNaturalLanguageAPIv1::AuthCloudLanguage => "https://www.googleapis.com/auth/cloud-language",
            CloudNaturalLanguageAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudOSLoginAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudOSLoginAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudOSLoginAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            CloudOSLoginAPIv1::AuthCompute => "https://www.googleapis.com/auth/compute",
            CloudOSLoginAPIv1::AuthComputeReadOnly => "https://www.googleapis.com/auth/compute.readonly",
            
        }
    }
}




impl ToGoogleScope for CloudProfilerAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudProfilerAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudProfilerAPIv2::AuthMonitoring => "https://www.googleapis.com/auth/monitoring",
            CloudProfilerAPIv2::AuthMonitoringWrite => "https://www.googleapis.com/auth/monitoring.write",
            
        }
    }
}




impl ToGoogleScope for CloudPub/SubAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudPub/SubAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudPub/SubAPIv1::AuthPubsub => "https://www.googleapis.com/auth/pubsub",
            
        }
    }
}




impl ToGoogleScope for CloudResourceManagerAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudResourceManagerAPIv3::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudResourceManagerAPIv3::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            
        }
    }
}




impl ToGoogleScope for CloudRuntimeConfigurationAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudRuntimeConfigurationAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudRuntimeConfigurationAPIv1::AuthCloudruntimeconfig => "https://www.googleapis.com/auth/cloudruntimeconfig",
            
        }
    }
}




impl ToGoogleScope for CloudSQLAdminAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudSQLAdminAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudSQLAdminAPIv1::AuthSqlserviceAdmin => "https://www.googleapis.com/auth/sqlservice.admin",
            
        }
    }
}




impl ToGoogleScope for CloudSchedulerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudSchedulerAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudSearchAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudSearchAPIv1::AuthCloud_Search => "https://www.googleapis.com/auth/cloud_search",
            CloudSearchAPIv1::AuthCloud_SearchDebug => "https://www.googleapis.com/auth/cloud_search.debug",
            CloudSearchAPIv1::AuthCloud_SearchIndexing => "https://www.googleapis.com/auth/cloud_search.indexing",
            CloudSearchAPIv1::AuthCloud_SearchQuery => "https://www.googleapis.com/auth/cloud_search.query",
            CloudSearchAPIv1::AuthCloud_SearchSettings => "https://www.googleapis.com/auth/cloud_search.settings",
            CloudSearchAPIv1::AuthCloud_SearchSettingsIndexing => "https://www.googleapis.com/auth/cloud_search.settings.indexing",
            CloudSearchAPIv1::AuthCloud_SearchSettingsQuery => "https://www.googleapis.com/auth/cloud_search.settings.query",
            CloudSearchAPIv1::AuthCloud_SearchStats => "https://www.googleapis.com/auth/cloud_search.stats",
            CloudSearchAPIv1::AuthCloud_SearchStatsIndexing => "https://www.googleapis.com/auth/cloud_search.stats.indexing",
            
        }
    }
}




impl ToGoogleScope for CloudShellAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudShellAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudSourceRepositoriesAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudSourceRepositoriesAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudSourceRepositoriesAPIv1::AuthSourceFull_Control => "https://www.googleapis.com/auth/source.full_control",
            CloudSourceRepositoriesAPIv1::AuthSourceRead_Only => "https://www.googleapis.com/auth/source.read_only",
            CloudSourceRepositoriesAPIv1::AuthSourceRead_Write => "https://www.googleapis.com/auth/source.read_write",
            
        }
    }
}




impl ToGoogleScope for CloudSpannerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudSpannerAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudSpannerAPIv1::AuthSpannerAdmin => "https://www.googleapis.com/auth/spanner.admin",
            CloudSpannerAPIv1::AuthSpannerData => "https://www.googleapis.com/auth/spanner.data",
            
        }
    }
}




impl ToGoogleScope for CloudSpeech-to-TextAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudSpeech-to-TextAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudStorageJSONAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudStorageJSONAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudStorageJSONAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            CloudStorageJSONAPIv1::AuthDevstorageFull_Control => "https://www.googleapis.com/auth/devstorage.full_control",
            CloudStorageJSONAPIv1::AuthDevstorageRead_Only => "https://www.googleapis.com/auth/devstorage.read_only",
            CloudStorageJSONAPIv1::AuthDevstorageRead_Write => "https://www.googleapis.com/auth/devstorage.read_write",
            
        }
    }
}




impl ToGoogleScope for CloudTasksAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudTasksAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudTestingAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudTestingAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudTestingAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            
        }
    }
}




impl ToGoogleScope for CloudText-to-SpeechAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudText-to-SpeechAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudToolResultsAPIv1beta3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudToolResultsAPIv1beta3::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudTraceAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudTraceAPIv2::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudTraceAPIv2::AuthTraceAppend => "https://www.googleapis.com/auth/trace.append",
            
        }
    }
}




impl ToGoogleScope for CloudTranslationAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudTranslationAPIv3::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudTranslationAPIv3::AuthCloudTranslation => "https://www.googleapis.com/auth/cloud-translation",
            
        }
    }
}




impl ToGoogleScope for CloudVideoIntelligenceAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudVideoIntelligenceAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for CloudVisionAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            CloudVisionAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            CloudVisionAPIv1::AuthCloudVision => "https://www.googleapis.com/auth/cloud-vision",
            
        }
    }
}




impl ToGoogleScope for ComputeEngineAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ComputeEngineAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            ComputeEngineAPIv1::AuthCompute => "https://www.googleapis.com/auth/compute",
            ComputeEngineAPIv1::AuthComputeReadOnly => "https://www.googleapis.com/auth/compute.readonly",
            ComputeEngineAPIv1::AuthDevstorageFull_Control => "https://www.googleapis.com/auth/devstorage.full_control",
            ComputeEngineAPIv1::AuthDevstorageRead_Only => "https://www.googleapis.com/auth/devstorage.read_only",
            ComputeEngineAPIv1::AuthDevstorageRead_Write => "https://www.googleapis.com/auth/devstorage.read_write",
            
        }
    }
}




impl ToGoogleScope for ContentAPIforShoppingv2Point1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ContentAPIforShoppingv2Point1::AuthContent => "https://www.googleapis.com/auth/content",
            
        }
    }
}




impl ToGoogleScope for DataPortabilityAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            DataPortabilityAPIv1::AuthDataportabilityBusinessmessagingConversations => "https://www.googleapis.com/auth/dataportability.businessmessaging.conversations",
            DataPortabilityAPIv1::AuthDataportabilityChromeAutofill => "https://www.googleapis.com/auth/dataportability.chrome.autofill",
            DataPortabilityAPIv1::AuthDataportabilityChromeBookmarks => "https://www.googleapis.com/auth/dataportability.chrome.bookmarks",
            DataPortabilityAPIv1::AuthDataportabilityChromeDictionary => "https://www.googleapis.com/auth/dataportability.chrome.dictionary",
            DataPortabilityAPIv1::AuthDataportabilityChromeExtensions => "https://www.googleapis.com/auth/dataportability.chrome.extensions",
            DataPortabilityAPIv1::AuthDataportabilityChromeHistory => "https://www.googleapis.com/auth/dataportability.chrome.history",
            DataPortabilityAPIv1::AuthDataportabilityChromeReading_List => "https://www.googleapis.com/auth/dataportability.chrome.reading_list",
            DataPortabilityAPIv1::AuthDataportabilityChromeSettings => "https://www.googleapis.com/auth/dataportability.chrome.settings",
            DataPortabilityAPIv1::AuthDataportabilityMapsCommute_Routes => "https://www.googleapis.com/auth/dataportability.maps.commute_routes",
            DataPortabilityAPIv1::AuthDataportabilityMapsCommute_Settings => "https://www.googleapis.com/auth/dataportability.maps.commute_settings",
            DataPortabilityAPIv1::AuthDataportabilityMapsEv_Profile => "https://www.googleapis.com/auth/dataportability.maps.ev_profile",
            DataPortabilityAPIv1::AuthDataportabilityMapsOffering_Contributions => "https://www.googleapis.com/auth/dataportability.maps.offering_contributions",
            DataPortabilityAPIv1::AuthDataportabilityMapsPhotos_Videos => "https://www.googleapis.com/auth/dataportability.maps.photos_videos",
            DataPortabilityAPIv1::AuthDataportabilityMapsReviews => "https://www.googleapis.com/auth/dataportability.maps.reviews",
            DataPortabilityAPIv1::AuthDataportabilityMapsStarred_Places => "https://www.googleapis.com/auth/dataportability.maps.starred_places",
            DataPortabilityAPIv1::AuthDataportabilityMyactivityMaps => "https://www.googleapis.com/auth/dataportability.myactivity.maps",
            DataPortabilityAPIv1::AuthDataportabilityMyactivitySearch => "https://www.googleapis.com/auth/dataportability.myactivity.search",
            DataPortabilityAPIv1::AuthDataportabilityMyactivityShopping => "https://www.googleapis.com/auth/dataportability.myactivity.shopping",
            DataPortabilityAPIv1::AuthDataportabilityMyactivityYoutube => "https://www.googleapis.com/auth/dataportability.myactivity.youtube",
            DataPortabilityAPIv1::AuthDataportabilitySavedCollections => "https://www.googleapis.com/auth/dataportability.saved.collections",
            DataPortabilityAPIv1::AuthDataportabilityShoppingAddresses => "https://www.googleapis.com/auth/dataportability.shopping.addresses",
            DataPortabilityAPIv1::AuthDataportabilityShoppingReviews => "https://www.googleapis.com/auth/dataportability.shopping.reviews",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeChannel => "https://www.googleapis.com/auth/dataportability.youtube.channel",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeComments => "https://www.googleapis.com/auth/dataportability.youtube.comments",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeLive_Chat => "https://www.googleapis.com/auth/dataportability.youtube.live_chat",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeMusic => "https://www.googleapis.com/auth/dataportability.youtube.music",
            DataPortabilityAPIv1::AuthDataportabilityYoutubePlayable => "https://www.googleapis.com/auth/dataportability.youtube.playable",
            DataPortabilityAPIv1::AuthDataportabilityYoutubePosts => "https://www.googleapis.com/auth/dataportability.youtube.posts",
            DataPortabilityAPIv1::AuthDataportabilityYoutubePrivate_Playlists => "https://www.googleapis.com/auth/dataportability.youtube.private_playlists",
            DataPortabilityAPIv1::AuthDataportabilityYoutubePrivate_Videos => "https://www.googleapis.com/auth/dataportability.youtube.private_videos",
            DataPortabilityAPIv1::AuthDataportabilityYoutubePublic_Playlists => "https://www.googleapis.com/auth/dataportability.youtube.public_playlists",
            DataPortabilityAPIv1::AuthDataportabilityYoutubePublic_Videos => "https://www.googleapis.com/auth/dataportability.youtube.public_videos",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeShopping => "https://www.googleapis.com/auth/dataportability.youtube.shopping",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeSubscriptions => "https://www.googleapis.com/auth/dataportability.youtube.subscriptions",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeUnlisted_Playlists => "https://www.googleapis.com/auth/dataportability.youtube.unlisted_playlists",
            DataPortabilityAPIv1::AuthDataportabilityYoutubeUnlisted_Videos => "https://www.googleapis.com/auth/dataportability.youtube.unlisted_videos",
            
        }
    }
}




impl ToGoogleScope for DataflowAPIv1b3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            DataflowAPIv1b3::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            DataflowAPIv1b3::AuthCompute => "https://www.googleapis.com/auth/compute",
            DataflowAPIv1b3::AuthComputeReadOnly => "https://www.googleapis.com/auth/compute.readonly",
            DataflowAPIv1b3::AuthUserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            
        }
    }
}




impl ToGoogleScope for DriveActivityAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            DriveActivityAPIv2::AuthDriveActivity => "https://www.googleapis.com/auth/drive.activity",
            DriveActivityAPIv2::AuthDriveActivityReadOnly => "https://www.googleapis.com/auth/drive.activity.readonly",
            
        }
    }
}




impl ToGoogleScope for EnterpriseLicenseManagerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            EnterpriseLicenseManagerAPIv1::AuthAppsLicensing => "https://www.googleapis.com/auth/apps.licensing",
            
        }
    }
}




impl ToGoogleScope for ErrorReportingAPIv1beta1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ErrorReportingAPIv1beta1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for FactCheckToolsAPIv1alpha1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            FactCheckToolsAPIv1alpha1::AuthUserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            
        }
    }
}




impl ToGoogleScope for FirebaseCloudMessagingAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            FirebaseCloudMessagingAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            FirebaseCloudMessagingAPIv1::AuthFirebaseMessaging => "https://www.googleapis.com/auth/firebase.messaging",
            
        }
    }
}




impl ToGoogleScope for FirebaseDynamicLinksAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            FirebaseDynamicLinksAPIv1::AuthFirebase => "https://www.googleapis.com/auth/firebase",
            
        }
    }
}




impl ToGoogleScope for FirebaseManagementAPIv1beta1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            FirebaseManagementAPIv1beta1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            FirebaseManagementAPIv1beta1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            FirebaseManagementAPIv1beta1::AuthFirebase => "https://www.googleapis.com/auth/firebase",
            FirebaseManagementAPIv1beta1::AuthFirebaseReadOnly => "https://www.googleapis.com/auth/firebase.readonly",
            
        }
    }
}




impl ToGoogleScope for FirebaseRulesAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            FirebaseRulesAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            FirebaseRulesAPIv1::AuthFirebase => "https://www.googleapis.com/auth/firebase",
            FirebaseRulesAPIv1::AuthFirebaseReadOnly => "https://www.googleapis.com/auth/firebase.readonly",
            
        }
    }
}




impl ToGoogleScope for FitnessAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            FitnessAPIv1::AuthFitnessActivityRead => "https://www.googleapis.com/auth/fitness.activity.read",
            FitnessAPIv1::AuthFitnessActivityWrite => "https://www.googleapis.com/auth/fitness.activity.write",
            FitnessAPIv1::AuthFitnessBlood_GlucoseRead => "https://www.googleapis.com/auth/fitness.blood_glucose.read",
            FitnessAPIv1::AuthFitnessBlood_GlucoseWrite => "https://www.googleapis.com/auth/fitness.blood_glucose.write",
            FitnessAPIv1::AuthFitnessBlood_PressureRead => "https://www.googleapis.com/auth/fitness.blood_pressure.read",
            FitnessAPIv1::AuthFitnessBlood_PressureWrite => "https://www.googleapis.com/auth/fitness.blood_pressure.write",
            FitnessAPIv1::AuthFitnessBodyRead => "https://www.googleapis.com/auth/fitness.body.read",
            FitnessAPIv1::AuthFitnessBodyWrite => "https://www.googleapis.com/auth/fitness.body.write",
            FitnessAPIv1::AuthFitnessBody_TemperatureRead => "https://www.googleapis.com/auth/fitness.body_temperature.read",
            FitnessAPIv1::AuthFitnessBody_TemperatureWrite => "https://www.googleapis.com/auth/fitness.body_temperature.write",
            FitnessAPIv1::AuthFitnessHeart_RateRead => "https://www.googleapis.com/auth/fitness.heart_rate.read",
            FitnessAPIv1::AuthFitnessHeart_RateWrite => "https://www.googleapis.com/auth/fitness.heart_rate.write",
            FitnessAPIv1::AuthFitnessLocationRead => "https://www.googleapis.com/auth/fitness.location.read",
            FitnessAPIv1::AuthFitnessLocationWrite => "https://www.googleapis.com/auth/fitness.location.write",
            FitnessAPIv1::AuthFitnessNutritionRead => "https://www.googleapis.com/auth/fitness.nutrition.read",
            FitnessAPIv1::AuthFitnessNutritionWrite => "https://www.googleapis.com/auth/fitness.nutrition.write",
            FitnessAPIv1::AuthFitnessOxygen_SaturationRead => "https://www.googleapis.com/auth/fitness.oxygen_saturation.read",
            FitnessAPIv1::AuthFitnessOxygen_SaturationWrite => "https://www.googleapis.com/auth/fitness.oxygen_saturation.write",
            FitnessAPIv1::AuthFitnessReproductive_HealthRead => "https://www.googleapis.com/auth/fitness.reproductive_health.read",
            FitnessAPIv1::AuthFitnessReproductive_HealthWrite => "https://www.googleapis.com/auth/fitness.reproductive_health.write",
            FitnessAPIv1::AuthFitnessSleepRead => "https://www.googleapis.com/auth/fitness.sleep.read",
            FitnessAPIv1::AuthFitnessSleepWrite => "https://www.googleapis.com/auth/fitness.sleep.write",
            
        }
    }
}




impl ToGoogleScope for GenomicsAPIv2alpha1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GenomicsAPIv2alpha1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            GenomicsAPIv2alpha1::AuthGenomics => "https://www.googleapis.com/auth/genomics",
            
        }
    }
}




impl ToGoogleScope for GmailAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GmailAPIv1::/ => "https://mail.google.com/",
            GmailAPIv1::AuthGmailAddonsCurrentActionCompose => "https://www.googleapis.com/auth/gmail.addons.current.action.compose",
            GmailAPIv1::AuthGmailAddonsCurrentMessageAction => "https://www.googleapis.com/auth/gmail.addons.current.message.action",
            GmailAPIv1::AuthGmailAddonsCurrentMessageMetadata => "https://www.googleapis.com/auth/gmail.addons.current.message.metadata",
            GmailAPIv1::AuthGmailAddonsCurrentMessageReadOnly => "https://www.googleapis.com/auth/gmail.addons.current.message.readonly",
            GmailAPIv1::AuthGmailCompose => "https://www.googleapis.com/auth/gmail.compose",
            GmailAPIv1::AuthGmailInsert => "https://www.googleapis.com/auth/gmail.insert",
            GmailAPIv1::AuthGmailLabels => "https://www.googleapis.com/auth/gmail.labels",
            GmailAPIv1::AuthGmailMetadata => "https://www.googleapis.com/auth/gmail.metadata",
            GmailAPIv1::AuthGmailModify => "https://www.googleapis.com/auth/gmail.modify",
            GmailAPIv1::AuthGmailReadOnly => "https://www.googleapis.com/auth/gmail.readonly",
            GmailAPIv1::AuthGmailSend => "https://www.googleapis.com/auth/gmail.send",
            GmailAPIv1::AuthGmailSettingsBasic => "https://www.googleapis.com/auth/gmail.settings.basic",
            GmailAPIv1::AuthGmailSettingsSharing => "https://www.googleapis.com/auth/gmail.settings.sharing",
            
        }
    }
}




impl ToGoogleScope for GoogleAnalyticsAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleAnalyticsAPIv3::AuthAnalytics => "https://www.googleapis.com/auth/analytics",
            GoogleAnalyticsAPIv3::AuthAnalyticsEdit => "https://www.googleapis.com/auth/analytics.edit",
            GoogleAnalyticsAPIv3::AuthAnalyticsManageUsers => "https://www.googleapis.com/auth/analytics.manage.users",
            GoogleAnalyticsAPIv3::AuthAnalyticsManageUsersReadOnly => "https://www.googleapis.com/auth/analytics.manage.users.readonly",
            GoogleAnalyticsAPIv3::AuthAnalyticsProvision => "https://www.googleapis.com/auth/analytics.provision",
            GoogleAnalyticsAPIv3::AuthAnalyticsReadOnly => "https://www.googleapis.com/auth/analytics.readonly",
            GoogleAnalyticsAPIv3::AuthAnalyticsUserDeletion => "https://www.googleapis.com/auth/analytics.user.deletion",
            
        }
    }
}




impl ToGoogleScope for GoogleChatAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleChatAPIv1::AuthChatDelete => "https://www.googleapis.com/auth/chat.delete",
            GoogleChatAPIv1::AuthChatMemberships => "https://www.googleapis.com/auth/chat.memberships",
            GoogleChatAPIv1::AuthChatMembershipsApp => "https://www.googleapis.com/auth/chat.memberships.app",
            GoogleChatAPIv1::AuthChatMembershipsReadOnly => "https://www.googleapis.com/auth/chat.memberships.readonly",
            GoogleChatAPIv1::AuthChatMessages => "https://www.googleapis.com/auth/chat.messages",
            GoogleChatAPIv1::AuthChatMessagesCreate => "https://www.googleapis.com/auth/chat.messages.create",
            GoogleChatAPIv1::AuthChatMessagesReactions => "https://www.googleapis.com/auth/chat.messages.reactions",
            GoogleChatAPIv1::AuthChatMessagesReactionsCreate => "https://www.googleapis.com/auth/chat.messages.reactions.create",
            GoogleChatAPIv1::AuthChatMessagesReactionsReadOnly => "https://www.googleapis.com/auth/chat.messages.reactions.readonly",
            GoogleChatAPIv1::AuthChatMessagesReadOnly => "https://www.googleapis.com/auth/chat.messages.readonly",
            GoogleChatAPIv1::AuthChatSpaces => "https://www.googleapis.com/auth/chat.spaces",
            GoogleChatAPIv1::AuthChatSpacesCreate => "https://www.googleapis.com/auth/chat.spaces.create",
            GoogleChatAPIv1::AuthChatSpacesReadOnly => "https://www.googleapis.com/auth/chat.spaces.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleClassroomAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleClassroomAPIv1::AuthClassroomAnnouncements => "https://www.googleapis.com/auth/classroom.announcements",
            GoogleClassroomAPIv1::AuthClassroomAnnouncementsReadOnly => "https://www.googleapis.com/auth/classroom.announcements.readonly",
            GoogleClassroomAPIv1::AuthClassroomCourses => "https://www.googleapis.com/auth/classroom.courses",
            GoogleClassroomAPIv1::AuthClassroomCoursesReadOnly => "https://www.googleapis.com/auth/classroom.courses.readonly",
            GoogleClassroomAPIv1::AuthClassroomCourseworkMe => "https://www.googleapis.com/auth/classroom.coursework.me",
            GoogleClassroomAPIv1::AuthClassroomCourseworkMeReadOnly => "https://www.googleapis.com/auth/classroom.coursework.me.readonly",
            GoogleClassroomAPIv1::AuthClassroomCourseworkStudents => "https://www.googleapis.com/auth/classroom.coursework.students",
            GoogleClassroomAPIv1::AuthClassroomCourseworkStudentsReadOnly => "https://www.googleapis.com/auth/classroom.coursework.students.readonly",
            GoogleClassroomAPIv1::AuthClassroomCourseworkmaterials => "https://www.googleapis.com/auth/classroom.courseworkmaterials",
            GoogleClassroomAPIv1::AuthClassroomCourseworkmaterialsReadOnly => "https://www.googleapis.com/auth/classroom.courseworkmaterials.readonly",
            GoogleClassroomAPIv1::AuthClassroomGuardianlinksMeReadOnly => "https://www.googleapis.com/auth/classroom.guardianlinks.me.readonly",
            GoogleClassroomAPIv1::AuthClassroomGuardianlinksStudents => "https://www.googleapis.com/auth/classroom.guardianlinks.students",
            GoogleClassroomAPIv1::AuthClassroomGuardianlinksStudentsReadOnly => "https://www.googleapis.com/auth/classroom.guardianlinks.students.readonly",
            GoogleClassroomAPIv1::AuthClassroomProfileEmails => "https://www.googleapis.com/auth/classroom.profile.emails",
            GoogleClassroomAPIv1::AuthClassroomProfilePhotos => "https://www.googleapis.com/auth/classroom.profile.photos",
            GoogleClassroomAPIv1::AuthClassroomPushNotifications => "https://www.googleapis.com/auth/classroom.push-notifications",
            GoogleClassroomAPIv1::AuthClassroomRosters => "https://www.googleapis.com/auth/classroom.rosters",
            GoogleClassroomAPIv1::AuthClassroomRostersReadOnly => "https://www.googleapis.com/auth/classroom.rosters.readonly",
            GoogleClassroomAPIv1::AuthClassroomStudentSubmissionsMeReadOnly => "https://www.googleapis.com/auth/classroom.student-submissions.me.readonly",
            GoogleClassroomAPIv1::AuthClassroomStudentSubmissionsStudentsReadOnly => "https://www.googleapis.com/auth/classroom.student-submissions.students.readonly",
            GoogleClassroomAPIv1::AuthClassroomTopics => "https://www.googleapis.com/auth/classroom.topics",
            GoogleClassroomAPIv1::AuthClassroomTopicsReadOnly => "https://www.googleapis.com/auth/classroom.topics.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleCloudDataCatalogAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleCloudDataCatalogAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for GoogleCloudMemorystoreforRedisAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleCloudMemorystoreforRedisAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for GoogleDocsAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleDocsAPIv1::AuthDocuments => "https://www.googleapis.com/auth/documents",
            GoogleDocsAPIv1::AuthDocumentsReadOnly => "https://www.googleapis.com/auth/documents.readonly",
            GoogleDocsAPIv1::AuthDrive => "https://www.googleapis.com/auth/drive",
            GoogleDocsAPIv1::AuthDriveFile => "https://www.googleapis.com/auth/drive.file",
            GoogleDocsAPIv1::AuthDriveReadOnly => "https://www.googleapis.com/auth/drive.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleDriveAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleDriveAPIv3::AuthDrive => "https://www.googleapis.com/auth/drive",
            GoogleDriveAPIv3::AuthDriveAppdata => "https://www.googleapis.com/auth/drive.appdata",
            GoogleDriveAPIv3::AuthDriveFile => "https://www.googleapis.com/auth/drive.file",
            GoogleDriveAPIv3::AuthDriveMetadata => "https://www.googleapis.com/auth/drive.metadata",
            GoogleDriveAPIv3::AuthDriveMetadataReadOnly => "https://www.googleapis.com/auth/drive.metadata.readonly",
            GoogleDriveAPIv3::AuthDrivePhotosReadOnly => "https://www.googleapis.com/auth/drive.photos.readonly",
            GoogleDriveAPIv3::AuthDriveReadOnly => "https://www.googleapis.com/auth/drive.readonly",
            GoogleDriveAPIv3::AuthDriveScripts => "https://www.googleapis.com/auth/drive.scripts",
            
        }
    }
}




impl ToGoogleScope for GoogleIdentityToolkitAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleIdentityToolkitAPIv3::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            GoogleIdentityToolkitAPIv3::AuthFirebase => "https://www.googleapis.com/auth/firebase",
            
        }
    }
}




impl ToGoogleScope for GoogleOAuth2APIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleOAuth2APIv2::AuthUserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            GoogleOAuth2APIv2::AuthUserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
            
        }
    }
}




impl ToGoogleScope for GooglePlayAndroidDeveloperAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GooglePlayAndroidDeveloperAPIv3::AuthAndroidpublisher => "https://www.googleapis.com/auth/androidpublisher",
            
        }
    }
}




impl ToGoogleScope for GooglePlayCustomAppPublishingAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GooglePlayCustomAppPublishingAPIv1::AuthAndroidpublisher => "https://www.googleapis.com/auth/androidpublisher",
            
        }
    }
}




impl ToGoogleScope for GooglePlayEMMAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GooglePlayEMMAPIv1::AuthAndroidenterprise => "https://www.googleapis.com/auth/androidenterprise",
            GooglePlayEMMAPIv1::AuthGames => "https://www.googleapis.com/auth/games",
            GooglePlayEMMAPIv1::AuthDriveAppdata => "https://www.googleapis.com/auth/drive.appdata",
            GooglePlayEMMAPIv1::AuthGames => "https://www.googleapis.com/auth/games",
            
        }
    }
}




impl ToGoogleScope for GooglePlayGameServicesPublishingAPIv1configuration {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GooglePlayGameServicesPublishingAPIv1configuration::AuthAndroidpublisher => "https://www.googleapis.com/auth/androidpublisher",
            
        }
    }
}




impl ToGoogleScope for GoogleSearchConsoleAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleSearchConsoleAPIv1::AuthWebmasters => "https://www.googleapis.com/auth/webmasters",
            GoogleSearchConsoleAPIv1::AuthWebmastersReadOnly => "https://www.googleapis.com/auth/webmasters.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleSheetsAPIv4 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleSheetsAPIv4::AuthDrive => "https://www.googleapis.com/auth/drive",
            GoogleSheetsAPIv4::AuthDriveFile => "https://www.googleapis.com/auth/drive.file",
            GoogleSheetsAPIv4::AuthDriveReadOnly => "https://www.googleapis.com/auth/drive.readonly",
            GoogleSheetsAPIv4::AuthSpreadsheets => "https://www.googleapis.com/auth/spreadsheets",
            GoogleSheetsAPIv4::AuthSpreadsheetsReadOnly => "https://www.googleapis.com/auth/spreadsheets.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleSiteVerificationAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleSiteVerificationAPIv1::AuthSiteverification => "https://www.googleapis.com/auth/siteverification",
            GoogleSiteVerificationAPIv1::AuthSiteverificationVerify_Only => "https://www.googleapis.com/auth/siteverification.verify_only",
            
        }
    }
}




impl ToGoogleScope for GoogleSlidesAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleSlidesAPIv1::AuthDrive => "https://www.googleapis.com/auth/drive",
            GoogleSlidesAPIv1::AuthDriveFile => "https://www.googleapis.com/auth/drive.file",
            GoogleSlidesAPIv1::AuthDriveReadOnly => "https://www.googleapis.com/auth/drive.readonly",
            GoogleSlidesAPIv1::AuthPresentations => "https://www.googleapis.com/auth/presentations",
            GoogleSlidesAPIv1::AuthPresentationsReadOnly => "https://www.googleapis.com/auth/presentations.readonly",
            GoogleSlidesAPIv1::AuthSpreadsheets => "https://www.googleapis.com/auth/spreadsheets",
            GoogleSlidesAPIv1::AuthSpreadsheetsReadOnly => "https://www.googleapis.com/auth/spreadsheets.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleTasksAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleTasksAPIv1::AuthTasks => "https://www.googleapis.com/auth/tasks",
            GoogleTasksAPIv1::AuthTasksReadOnly => "https://www.googleapis.com/auth/tasks.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleVaultAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleVaultAPIv1::AuthEdiscovery => "https://www.googleapis.com/auth/ediscovery",
            GoogleVaultAPIv1::AuthEdiscoveryReadOnly => "https://www.googleapis.com/auth/ediscovery.readonly",
            
        }
    }
}




impl ToGoogleScope for GoogleWorkspaceAlertCenterAPIv1beta1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleWorkspaceAlertCenterAPIv1beta1::AuthAppsAlerts => "https://www.googleapis.com/auth/apps.alerts",
            
        }
    }
}




impl ToGoogleScope for GoogleWorkspaceResellerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GoogleWorkspaceResellerAPIv1::AuthAppsOrder => "https://www.googleapis.com/auth/apps.order",
            GoogleWorkspaceResellerAPIv1::AuthAppsOrderReadOnly => "https://www.googleapis.com/auth/apps.order.readonly",
            
        }
    }
}




impl ToGoogleScope for GroupsMigrationAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GroupsMigrationAPIv1::AuthAppsGroupsMigration => "https://www.googleapis.com/auth/apps.groups.migration",
            
        }
    }
}




impl ToGoogleScope for GroupsSettingsAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            GroupsSettingsAPIv1::AuthAppsGroupsSettings => "https://www.googleapis.com/auth/apps.groups.settings",
            
        }
    }
}




impl ToGoogleScope for IAMServiceAccountCredentialsAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            IAMServiceAccountCredentialsAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for IndexingAPIv3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            IndexingAPIv3::AuthIndexing => "https://www.googleapis.com/auth/indexing",
            
        }
    }
}




impl ToGoogleScope for KubernetesEngineAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            KubernetesEngineAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for LibraryAgentAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            LibraryAgentAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for ManagedServiceforMicrosoftActiveDirectoryAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ManagedServiceforMicrosoftActiveDirectoryAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for ManufacturerCenterAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ManufacturerCenterAPIv1::AuthManufacturercenter => "https://www.googleapis.com/auth/manufacturercenter",
            
        }
    }
}




impl ToGoogleScope for NetworkManagementAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            NetworkManagementAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for OSConfigAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            OSConfigAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for PeopleAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            PeopleAPIv1::AuthContacts => "https://www.googleapis.com/auth/contacts",
            PeopleAPIv1::AuthContactsOtherReadOnly => "https://www.googleapis.com/auth/contacts.other.readonly",
            PeopleAPIv1::AuthContactsReadOnly => "https://www.googleapis.com/auth/contacts.readonly",
            PeopleAPIv1::AuthDirectoryReadOnly => "https://www.googleapis.com/auth/directory.readonly",
            PeopleAPIv1::AuthUserAddressesRead => "https://www.googleapis.com/auth/user.addresses.read",
            PeopleAPIv1::AuthUserBirthdayRead => "https://www.googleapis.com/auth/user.birthday.read",
            PeopleAPIv1::AuthUserEmailsRead => "https://www.googleapis.com/auth/user.emails.read",
            PeopleAPIv1::AuthUserGenderRead => "https://www.googleapis.com/auth/user.gender.read",
            PeopleAPIv1::AuthUserOrganizationRead => "https://www.googleapis.com/auth/user.organization.read",
            PeopleAPIv1::AuthUserPhonenumbersRead => "https://www.googleapis.com/auth/user.phonenumbers.read",
            PeopleAPIv1::AuthUserinfoEmail => "https://www.googleapis.com/auth/userinfo.email",
            PeopleAPIv1::AuthUserinfoProfile => "https://www.googleapis.com/auth/userinfo.profile",
            
        }
    }
}




impl ToGoogleScope for PhotosLibraryAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            PhotosLibraryAPIv1::AuthPhotoslibrary => "https://www.googleapis.com/auth/photoslibrary",
            PhotosLibraryAPIv1::AuthPhotoslibraryAppendOnly => "https://www.googleapis.com/auth/photoslibrary.appendonly",
            PhotosLibraryAPIv1::AuthPhotoslibraryEditAppcreateddata => "https://www.googleapis.com/auth/photoslibrary.edit.appcreateddata",
            PhotosLibraryAPIv1::AuthPhotoslibraryReadOnly => "https://www.googleapis.com/auth/photoslibrary.readonly",
            PhotosLibraryAPIv1::AuthPhotoslibraryReadOnlyAppcreateddata => "https://www.googleapis.com/auth/photoslibrary.readonly.appcreateddata",
            PhotosLibraryAPIv1::AuthPhotoslibrarySharing => "https://www.googleapis.com/auth/photoslibrary.sharing",
            
        }
    }
}




impl ToGoogleScope for PolicyTroubleshooterAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            PolicyTroubleshooterAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for RecommenderAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            RecommenderAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for SASPortalAPI(Testing)v1alpha1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            SASPortalAPI(Testing)v1alpha1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            SASPortalAPI(Testing)v1alpha1::AuthSasportal => "https://www.googleapis.com/auth/sasportal",
            
        }
    }
}




impl ToGoogleScope for SASPortalAPIv1alpha1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            SASPortalAPIv1alpha1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            SASPortalAPIv1alpha1::AuthSasportal => "https://www.googleapis.com/auth/sasportal",
            
        }
    }
}




impl ToGoogleScope for SearchAds360APIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            SearchAds360APIv2::AuthDoubleclicksearch => "https://www.googleapis.com/auth/doubleclicksearch",
            
        }
    }
}




impl ToGoogleScope for SecretManagerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            SecretManagerAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for SecurityCommandCenterAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            SecurityCommandCenterAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for ServerlessVPCAccessAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ServerlessVPCAccessAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for ServiceConsumerManagementAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ServiceConsumerManagementAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for ServiceManagementAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ServiceManagementAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            ServiceManagementAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            ServiceManagementAPIv1::AuthServiceManagement => "https://www.googleapis.com/auth/service.management",
            ServiceManagementAPIv1::AuthServiceManagementReadOnly => "https://www.googleapis.com/auth/service.management.readonly",
            
        }
    }
}




impl ToGoogleScope for ServiceNetworkingAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ServiceNetworkingAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            ServiceNetworkingAPIv1::AuthServiceManagement => "https://www.googleapis.com/auth/service.management",
            
        }
    }
}




impl ToGoogleScope for ServiceUsageAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            ServiceUsageAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            ServiceUsageAPIv1::AuthCloudPlatformReadOnly => "https://www.googleapis.com/auth/cloud-platform.read-only",
            ServiceUsageAPIv1::AuthServiceManagement => "https://www.googleapis.com/auth/service.management",
            
        }
    }
}




impl ToGoogleScope for StorageTransferAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            StorageTransferAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for StreetViewPublishAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            StreetViewPublishAPIv1::AuthStreetviewpublish => "https://www.googleapis.com/auth/streetviewpublish",
            
        }
    }
}




impl ToGoogleScope for TagManagerAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            TagManagerAPIv2::AuthTagmanagerDeleteContainers => "https://www.googleapis.com/auth/tagmanager.delete.containers",
            TagManagerAPIv2::AuthTagmanagerEditContainers => "https://www.googleapis.com/auth/tagmanager.edit.containers",
            TagManagerAPIv2::AuthTagmanagerEditContainerversions => "https://www.googleapis.com/auth/tagmanager.edit.containerversions",
            TagManagerAPIv2::AuthTagmanagerManageAccounts => "https://www.googleapis.com/auth/tagmanager.manage.accounts",
            TagManagerAPIv2::AuthTagmanagerManageUsers => "https://www.googleapis.com/auth/tagmanager.manage.users",
            TagManagerAPIv2::AuthTagmanagerPublish => "https://www.googleapis.com/auth/tagmanager.publish",
            TagManagerAPIv2::AuthTagmanagerReadOnly => "https://www.googleapis.com/auth/tagmanager.readonly",
            
        }
    }
}




impl ToGoogleScope for WebSecurityScannerAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            WebSecurityScannerAPIv1::AuthCloudPlatform => "https://www.googleapis.com/auth/cloud-platform",
            
        }
    }
}




impl ToGoogleScope for YouTubeAnalyticsAPIv2 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            YouTubeAnalyticsAPIv2::AuthYoutube => "https://www.googleapis.com/auth/youtube",
            YouTubeAnalyticsAPIv2::AuthYoutubeReadOnly => "https://www.googleapis.com/auth/youtube.readonly",
            YouTubeAnalyticsAPIv2::AuthYoutubepartner => "https://www.googleapis.com/auth/youtubepartner",
            YouTubeAnalyticsAPIv2::AuthYtAnalyticsMonetaryReadOnly => "https://www.googleapis.com/auth/yt-analytics-monetary.readonly",
            YouTubeAnalyticsAPIv2::AuthYtAnalyticsReadOnly => "https://www.googleapis.com/auth/yt-analytics.readonly",
            
        }
    }
}




impl ToGoogleScope for YouTubeDataAPIv3v3 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            YouTubeDataAPIv3v3::AuthYoutube => "https://www.googleapis.com/auth/youtube",
            YouTubeDataAPIv3v3::AuthYoutubeChannelMembershipsCreator => "https://www.googleapis.com/auth/youtube.channel-memberships.creator",
            YouTubeDataAPIv3v3::AuthYoutubeForceSsl => "https://www.googleapis.com/auth/youtube.force-ssl",
            YouTubeDataAPIv3v3::AuthYoutubeReadOnly => "https://www.googleapis.com/auth/youtube.readonly",
            YouTubeDataAPIv3v3::AuthYoutubeUpload => "https://www.googleapis.com/auth/youtube.upload",
            YouTubeDataAPIv3v3::AuthYoutubepartner => "https://www.googleapis.com/auth/youtubepartner",
            YouTubeDataAPIv3v3::AuthYoutubepartnerChannelAudit => "https://www.googleapis.com/auth/youtubepartner-channel-audit",
            
        }
    }
}




impl ToGoogleScope for YouTubeReportingAPIv1 {
    fn to_google_scope(&self) -> &'static str {
        match self {
            YouTubeReportingAPIv1::AuthYtAnalyticsMonetaryReadOnly => "https://www.googleapis.com/auth/yt-analytics-monetary.readonly",
            YouTubeReportingAPIv1::AuthYtAnalyticsReadOnly => "https://www.googleapis.com/auth/yt-analytics.readonly",
            
        }
    }
}

