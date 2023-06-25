using Microsoft.EntityFrameworkCore;
using Saku.Models;
using Saku.ViewModels.Interfaces;

namespace Saku.Database;

public class SakuDbContext : DbContext
{
    private readonly ISakuConfig _config;

    public SakuDbContext(ISakuConfig config)
    {
        _config = config;
    }

    public DbSet<ChatContextModel> ChatContexts => Set<ChatContextModel>();
    public DbSet<UserModel> Users => Set<UserModel>();
    public DbSet<ChannelModel> Channels => Set<ChannelModel>();

    protected override void OnConfiguring(DbContextOptionsBuilder optionsBuilder)
    {
        var version = ServerVersion.AutoDetect(_config.ConnectionString);
        optionsBuilder.UseMySql(_config.ConnectionString, version);
    }

    protected override void OnModelCreating(ModelBuilder modelBuilder)
    {
        modelBuilder.ApplyConfigurationsFromAssembly(typeof(SakuDbContext).Assembly);
    }
}