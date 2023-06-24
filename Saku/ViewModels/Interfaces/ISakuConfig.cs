namespace Saku.ViewModels.Interfaces;

public interface ISakuConfig
{
    string BotToken { get; }
    string DevelopGuild { get; }
    string ConnectionString { get; }
    string OpenIaToken { get; }
}