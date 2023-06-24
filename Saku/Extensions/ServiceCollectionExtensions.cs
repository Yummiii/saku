using Config.Net;
using Lina.DynamicMapperConfiguration;
using Lina.DynamicServicesProvider;
using Microsoft.Extensions.DependencyInjection;
using Microsoft.Extensions.Logging;
using Saku.Database;
using Saku.ViewModels.Interfaces;

namespace Saku.Extensions;

public static class ServiceCollectionExtensions
{
    public static void AddBaseDependencies(this IServiceCollection services)
    {
        services.AddDbContext<SakuDbContext>();
        services.AddDynamicServices<Program>();
        services.AddDynamicMappers<Program>();
        services.AddLogging(builder => builder.AddConsole());
        services.AddConfiguration();
    }

    private static void AddConfiguration(this IServiceCollection services)
    {
        var configuration = new ConfigurationBuilder<ISakuConfig>()
            .UseEnvironmentVariables()
            .UseJsonFile("config.json");

        services.AddSingleton(configuration.Build());
    }
}