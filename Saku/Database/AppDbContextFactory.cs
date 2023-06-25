using Microsoft.EntityFrameworkCore.Design;
using Microsoft.Extensions.DependencyInjection;
using Saku.Extensions;

namespace Saku.Database;

public class AppDbContextFactory : IDesignTimeDbContextFactory<SakuDbContext>
{
    public SakuDbContext CreateDbContext(string[] args)
    {
        var dependenciesBuilder = new ServiceCollection();
        dependenciesBuilder.AddBaseDependencies();
        var dependencyInjection = dependenciesBuilder.BuildServiceProvider();

        return dependencyInjection.GetRequiredService<SakuDbContext>();
    }
}