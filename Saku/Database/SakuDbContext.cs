using Microsoft.EntityFrameworkCore;
using Saku.ViewModels.Interfaces;

namespace Saku.Database;

public class SakuDbContext : DbContext
{
    private readonly ISakuConfig _config;

    public SakuDbContext(ISakuConfig config)
    {
        _config = config;
    }

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