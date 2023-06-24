#region Usings

using Discord;
using Discord.Interactions;
using Discord.WebSocket;
using Microsoft.Extensions.DependencyInjection;
using Saku.Extensions;
using Saku.Services.Interfaces;
using Saku.ViewModels.Interfaces;

#endregion

var discordClientConfig = new DiscordSocketConfig
{
    GatewayIntents = GatewayIntents.All
};

var discordClient = new DiscordSocketClient(discordClientConfig);
var interactionService = new InteractionService(discordClient.Rest);

#region DependenciesStartup

var dependenciesBuilder = new ServiceCollection();
dependenciesBuilder.AddSingleton<IDiscordClient>(discordClient);
dependenciesBuilder.AddSingleton(interactionService);
dependenciesBuilder.AddBaseDependencies();

var dependencyInjection = dependenciesBuilder.BuildServiceProvider();

#endregion

#region BotStart

var configs = dependencyInjection.GetRequiredService<ISakuConfig>();
dependencyInjection.GetRequiredService<IEventLoaderService>().Initialize();

await interactionService.AddModulesAsync(typeof(Program).Assembly, dependencyInjection);
await discordClient.LoginAsync(TokenType.Bot, configs.BotToken);
await discordClient.StartAsync();
await Task.Delay(Timeout.Infinite);

#endregion