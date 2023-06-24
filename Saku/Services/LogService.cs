using Discord;
using Lina.DynamicServicesProvider.Attributes;
using Microsoft.Extensions.Logging;
using Saku.Services.Interfaces;

namespace Saku.Services;

[Service(typeof(ILogService))]
public class LogService : ILogService
{
    private readonly ILogger<LogService> _logger;

    public LogService(ILogger<LogService> logger)
    {
        _logger = logger;
    }

    public void DiscordLogWriter(LogMessage message)
    {
        if (message.Exception is not null)
        {
            if(message.Exception.Message.Contains("Expected SocketInteractionContext`1, got SocketInteractionContext"))
                return;
            
            _logger.LogError(message.Exception, "{}", message.Exception);
            return;
        }

        switch (message.Severity)
        {
            case LogSeverity.Info:
                _logger.LogInformation("{} - {}", message.Source, message.Message);
                break;
            default:
                _logger.LogWarning("{} - {}", message.Source, message.Message);
                break;
        }
    }
}