import { Menu, Settings, Search, Plus, LogOut, Star } from 'lucide-react';
import { Button } from '@/components/ui/button';
import { Input } from '@/components/ui/input';
import { useNavigate, useLocation } from 'react-router-dom';
import { useAuthContext } from '@/contexts/AuthContext';
import EntryOverview from './EntryOverview';
import { useDashboardContext } from '@/contexts/DashboardContext';
import { useEffect } from 'react';
import { Checkbox } from '@/components/ui/checkbox';
import CategoryList from './CategoryList';

const DashboardLayout = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { logout } = useAuthContext();
  const { 
    setSearchText, 
    totalCount, 
    categoryDistribution,
    isSidebarCollapsed,
    setIsSidebarCollapsed,
    vaultStatus,
    refreshEntries,
    searchParams,
    maxCount,
    setFavoritesOnly,
  } = useDashboardContext();

  // Handle user logout
  const handleLogout = async () => {
    try {
      await logout();
      navigate('/auth');
    } catch (error) {
      console.error('Failed to logout:', error);
    }
  };

  // Log dashboard state changes for debugging
  useEffect(() => {
    console.log('📊 Dashboard State:', {
      totalCount,
      categoriesCount: categoryDistribution.length
    });
  }, [totalCount, categoryDistribution]);

  // Refresh entries when returning to dashboard
  useEffect(() => {
    if (location.pathname === '/vault') {
      console.log('🔄 Refreshing entries on dashboard return');
      refreshEntries();
    }
  }, [location.pathname, refreshEntries]);

  return (
    <div className="min-h-screen bg-base-100 flex pt-8"> {/* Added pt-8 here */}
      {/* Sidebar */}
      <aside className={`bg-base-200/60 border-r border-base-300 transition-all duration-300 flex flex-col
        ${isSidebarCollapsed ? 'w-16' : 'w-64'}`}>
        {/* Sidebar Header */}
        <div className="h-16 flex items-center justify-between px-4 border-b border-base-300">
          {!isSidebarCollapsed && <span className="text-lg font-semibold">Password Vault</span>}
          <Button
            variant="ghost"
            size="sm"
            onClick={() => setIsSidebarCollapsed(!isSidebarCollapsed)}
          >
            <Menu className="h-5 w-5" />
          </Button>
        </div>

        {/* Sidebar Content */}
        <div className="flex-1 overflow-y-auto py-3 px-2">
          {!isSidebarCollapsed && (
            <div className="space-y-4">
              <div className="stats bg-base-300/50 shadow-sm p-3 rounded-lg">
                <div className="stat p-2">
                  <div className="stat-title text-sm">Total Entries</div>
                  <div className="stat-value text-xl text-center">{maxCount ?? 0}</div>
                </div>
              </div>
              
              {/* Integrated Category List Component */}
              <CategoryList maxCount={maxCount} />
            </div>
          )}
        </div>
      </aside>

      {/* Main Content Area */}
      <div className="flex-1 flex flex-col">
        {/* Top Header */}
        <header className="h-16 border-b border-base-300 bg-base-100 flex items-center justify-between px-4">
          {/* Search Bar */}
          <div className="flex-1 max-w-2xl flex items-center gap-4">
            <div className="relative flex-1">
              <Search className="absolute left-3 top-1/2 transform -translate-y-1/2 h-4 w-4 text-base-content/50" />
              <Input
                type="text"
                placeholder="Search passwords..."
                className="w-full pl-10 pr-4"
                onChange={(e) => setSearchText(e.target.value)}
              />
            </div>
            <div className="flex items-center gap-3 px-3 py-2">
              <Checkbox
                id="favorites"
                checked={searchParams.favoritesOnly}
                onCheckedChange={(checked) => setFavoritesOnly(checked === true)}
                className="h-5 w-5"
              />
              <label
                htmlFor="favorites"
                className="text-base font-medium flex items-center gap-2 cursor-pointer select-none"
              >
                <Star className="h-5 w-5 text-warning" />
                Favorites
              </label>
            </div>
          </div>

          {/* Header Actions */}
          <div className="flex items-center gap-2 ml-4">
            <Button className="btn-secondary" size="sm" onClick={() => navigate('/entry/new')}>
              <Plus className="h-4 w-4 mr-2" />
              New Entry
            </Button>
            <Button 
              variant="ghost" 
              size="sm" 
              onClick={() => navigate('/settings')}
              className="flex items-center text-base"
            >
              <Settings className="h-5 w-5" />
            </Button>
            <Button 
              variant="ghost" 
              size="sm" 
              onClick={handleLogout}
              className="flex items-center text-base"
            >
              <LogOut className="h-5 w-5 mr-2" />
              Logout
            </Button>
          </div>
        </header>

        {/* Main Content */}
        <main className="flex-1 p-6 bg-base-200">
          {/* Vault Status Card */}
          <div className="mb-6 card bg-base-100 shadow-lg">
            {vaultStatus && (
              <div className="card-body">
                <h3 className="card-title text-lg">Vault Status</h3>
                <div className="grid grid-cols-3 gap-4">
                  <div>
                    Session Active: 
                    <span className={vaultStatus.sessionActive ? 'text-green-500' : 'text-red-500'}>
                      {' '}{vaultStatus.sessionActive ? '✓' : '✗'}
                    </span>
                  </div>
                  <div>
                    Key Hierarchy: 
                    <span className={vaultStatus.keyHierarchyPresent ? 'text-green-500' : 'text-red-500'}>
                      {' '}{vaultStatus.keyHierarchyPresent ? '✓' : '✗'}
                    </span>
                  </div>
                  <div>
                    Vault Manager: 
                    <span className={vaultStatus.vaultManagerPresent ? 'text-green-500' : 'text-red-500'}>
                      {' '}{vaultStatus.vaultManagerPresent ? '✓' : '✗'}
                    </span>
                  </div>
                </div>
              </div>
            )}
          </div>

          {/* Entries Overview */}
          <div className="card bg-base-100/50 shadow-lg">
            <div className="card-body h-[calc(100vh-19.5rem)] p-0">
              <EntryOverview />
            </div>
          </div>
        </main>
      </div>
    </div>
  );
};

export default DashboardLayout;